use crate::config::WebConfig;
use crate::llm::Message;
use futures_util::StreamExt;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug)]
struct ToolCall {
    name: String,
    parameters: serde_json::Value,
}

impl ToolCall {
    fn into_tool(self) -> Option<Tool> {
        match self.name.as_str() {
            "READ_FILE" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                Some(Tool::ReadFile { path })
            }
            "WRITE_FILE" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                let content = self.parameters.get("content")?.as_str()?.to_string();
                Some(Tool::WriteFile { path, content })
            }
            "APPEND_FILE" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                let content = self.parameters.get("content")?.as_str()?.to_string();
                Some(Tool::AppendFile { path, content })
            }
            "SEARCH_REPLACE" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                let old_string = self.parameters.get("old_string")?.as_str()?.to_string();
                let new_string = self.parameters.get("new_string")?.as_str()?.to_string();
                Some(Tool::SearchReplace {
                    path,
                    old_string,
                    new_string,
                })
            }
            "DELETE_FILE" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                Some(Tool::DeleteFile { path })
            }
            "LIST_FILES" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                Some(Tool::ListFiles { path })
            }
            "LIST_FILES_RECURSIVE" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                Some(Tool::ListFilesRecursive { path })
            }
            "CREATE_DIRECTORY" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                Some(Tool::CreateDirectory { path })
            }
            "GREP_SEARCH" => {
                let pattern = self.parameters.get("pattern")?.as_str()?.to_string();
                let path = self
                    .parameters
                    .get("path")
                    .and_then(|p| p.as_str())
                    .map(|s| s.to_string());
                Some(Tool::GrepSearch { pattern, path })
            }
            "GLOB_SEARCH" => {
                let pattern = self.parameters.get("pattern")?.as_str()?.to_string();
                Some(Tool::GlobSearch { pattern })
            }
            "READ_URL" => {
                let url = self.parameters.get("url")?.as_str()?.to_string();
                Some(Tool::ReadUrl { url })
            }
            "SEARCH_WEB" => {
                let query = self.parameters.get("query")?.as_str()?.to_string();
                Some(Tool::SearchWeb { query })
            }
            "REMEMBER" => {
                let fact = self.parameters.get("fact")?.as_str()?.to_string();
                Some(Tool::Remember { fact })
            }
            "RECALL" => Some(Tool::Recall),
            "INDEX_CODEBASE" => {
                let path = self.parameters.get("path")?.as_str()?.to_string();
                Some(Tool::IndexCodebase { path })
            }
            "SEARCH_INDEX" => {
                let query = self.parameters.get("query")?.as_str()?.to_string();
                Some(Tool::SearchIndex { query })
            }
            "FUZZY_FIND" => {
                let pattern = self.parameters.get("pattern")?.as_str()?.to_string();
                let path = self
                    .parameters
                    .get("path")
                    .and_then(|p| p.as_str())
                    .map(|s| s.to_string());
                Some(Tool::FuzzyFind { pattern, path })
            }
            "EXECUTE_CODE" => {
                let language = self.parameters.get("language")?.as_str()?.to_string();
                let code = self.parameters.get("code")?.as_str()?.to_string();
                Some(Tool::ExecuteCode { language, code })
            }
            "RUN_COMMAND" => {
                let command = self.parameters.get("command")?.as_str()?.to_string();
                Some(Tool::RunCommand { command })
            }
            "GIT_STATUS" => Some(Tool::GitStatus),
            "GIT_DIFF" => Some(Tool::GitDiff),
            "GIT_COMMIT" => {
                let message = self.parameters.get("message")?.as_str()?.to_string();
                Some(Tool::GitCommit { message })
            }
            "GIT_LOG" => {
                let count = self
                    .parameters
                    .get("count")
                    .and_then(|c| c.as_u64())
                    .map(|c| c as usize);
                Some(Tool::GitLog { count })
            }
            "RUN_LINT" => {
                let language = self.parameters.get("language")?.as_str()?.to_string();
                let path = self
                    .parameters
                    .get("path")
                    .and_then(|p| p.as_str())
                    .map(|s| s.to_string());
                Some(Tool::RunLint { language, path })
            }
            "RUN_TESTS" => {
                let framework = self.parameters.get("framework")?.as_str()?.to_string();
                let path = self
                    .parameters
                    .get("path")
                    .and_then(|p| p.as_str())
                    .map(|s| s.to_string());
                Some(Tool::RunTests { framework, path })
            }
            "INSTALL_PACKAGE" => {
                let manager = self.parameters.get("manager")?.as_str()?.to_string();
                let package = self.parameters.get("package")?.as_str()?.to_string();
                Some(Tool::InstallPackage { manager, package })
            }
            "CREATE_PLAN" => {
                let task = self.parameters.get("task")?.as_str()?.to_string();
                let steps_array = self.parameters.get("steps")?.as_array()?;
                let steps: Vec<String> = steps_array
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect();
                Some(Tool::CreatePlan { task, steps })
            }
            "UPDATE_PLAN" => {
                let completed_step = self.parameters.get("completed_step")?.as_u64()? as usize;
                Some(Tool::UpdatePlan { completed_step })
            }
            "CLEAR_PLAN" => Some(Tool::ClearPlan),
            "GET_TIME" => Some(Tool::GetTime),
            "GET_OS_INFO" => Some(Tool::GetOsInfo),
            "COPY_FILE" => {
                let source = self.parameters.get("source")?.as_str()?.to_string();
                let destination = self.parameters.get("destination")?.as_str()?.to_string();
                Some(Tool::CopyFile {
                    source,
                    destination,
                })
            }
            "MOVE_FILE" => {
                let source = self.parameters.get("source")?.as_str()?.to_string();
                let destination = self.parameters.get("destination")?.as_str()?.to_string();
                Some(Tool::MoveFile {
                    source,
                    destination,
                })
            }
            "RENAME_FILE" => {
                let old_name = self.parameters.get("old_name")?.as_str()?.to_string();
                let new_name = self.parameters.get("new_name")?.as_str()?.to_string();
                Some(Tool::RenameFile { old_name, new_name })
            }
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Tool {
    // File Operations
    ReadFile {
        path: String,
    },
    WriteFile {
        path: String,
        content: String,
    },
    AppendFile {
        path: String,
        content: String,
    },
    SearchReplace {
        path: String,
        old_string: String,
        new_string: String,
    },
    DeleteFile {
        path: String,
    },

    // Directory Operations
    ListFiles {
        path: String,
    },
    ListFilesRecursive {
        path: String,
    },
    CreateDirectory {
        path: String,
    },

    // Search & Navigation
    GrepSearch {
        pattern: String,
        path: Option<String>,
    },
    GlobSearch {
        pattern: String,
    },
    IndexCodebase {
        path: String,
    },
    SearchIndex {
        query: String,
    },
    FuzzyFind {
        pattern: String,
        path: Option<String>,
    },
    ReadUrl {
        url: String,
    },
    SearchWeb {
        query: String,
    },
    Remember {
        fact: String,
    },
    Recall,

    // Code Execution & Compilation
    ExecuteCode {
        language: String,
        code: String,
    },
    RunCommand {
        command: String,
    },

    // Development Workflow
    GitStatus,
    GitDiff,
    GitCommit {
        message: String,
    },
    GitLog {
        count: Option<usize>,
    },

    // Quality Assurance
    RunLint {
        language: String,
        path: Option<String>,
    },
    RunTests {
        framework: String,
        path: Option<String>,
    },

    // Package Management
    InstallPackage {
        manager: String,
        package: String,
    },

    // Planning and Task Management
    CreatePlan {
        task: String,
        steps: Vec<String>,
    },
    UpdatePlan {
        completed_step: usize,
    },
    ClearPlan,

    // System Information & Time
    GetTime,
    GetOsInfo,

    // Enhanced File Operations
    CopyFile {
        source: String,
        destination: String,
    },
    MoveFile {
        source: String,
        destination: String,
    },
    RenameFile {
        old_name: String,
        new_name: String,
    },
}

impl Tool {
    pub fn execute(&self, web_config: &WebConfig) -> Result<String, io::Error> {
        match self {
            // File Operations
            Tool::ReadFile { path } => {
                let content = fs::read_to_string(path)?;
                Ok(format!("File: {}\n\n{}", path, content))
            }
            Tool::WriteFile { path, content } => {
                // Create parent directories if they don't exist
                if let Some(parent) = Path::new(path).parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(path, content)?;
                Ok(format!("File '{}' written successfully ({} bytes).", path, content.len()))
            }
            Tool::AppendFile { path, content } => {
                if !Path::new(path).exists() {
                    return Err(io::Error::new(io::ErrorKind::NotFound, format!("File '{}' does not exist", path)));
                }
                let mut file = fs::OpenOptions::new().append(true).open(path)?;
                use std::io::Write;
                file.write_all(content.as_bytes())?;
                Ok(format!("Content appended to '{}' successfully ({} bytes added).", path, content.len()))
            }
            Tool::SearchReplace { path, old_string, new_string } => {
                let content = fs::read_to_string(path)?;
                if !content.contains(old_string) {
                    return Err(io::Error::new(io::ErrorKind::InvalidData,
                        format!("Old string '{}' not found in file '{}'", old_string, path)));
                }
                let new_content = content.replace(old_string, new_string);
                fs::write(path, new_content)?;
                Ok(format!("Successfully replaced '{}' with '{}' in '{}'", old_string, new_string, path))
            }
            Tool::DeleteFile { path } => {
                if Path::new(path).is_dir() {
                    fs::remove_dir_all(path)?;
                    Ok(format!("Directory '{}' deleted successfully.", path))
                } else {
                    fs::remove_file(path)?;
                    Ok(format!("File '{}' deleted successfully.", path))
                }
            }

            // Directory Operations
            Tool::ListFiles { path } => {
                let entries = fs::read_dir(path)?;
                let mut files = Vec::new();
                for entry in entries {
                    let entry = entry?;
                    let name = entry.file_name().to_string_lossy().to_string();
                    let is_dir = entry.file_type()?.is_dir();
                    files.push(format!("{}{}", if is_dir { "[DIR] " } else { "[FILE] " }, name));
                }
                files.sort();
                Ok(format!("Contents of '{}':\n{}", path, files.join("\n")))
            }
            Tool::ListFilesRecursive { path } => {
                fn collect_files(dir: &Path, prefix: &str) -> Result<Vec<String>, io::Error> {
                    let mut results = Vec::new();
                    let entries = fs::read_dir(dir)?;
                    for entry in entries {
                        let entry = entry?;
                        let name = entry.file_name().to_string_lossy().to_string();
                        let full_path = entry.path();
                        let is_dir = entry.file_type()?.is_dir();

                        if is_dir {
                            results.push(format!("{}[DIR] {}{}", prefix, name, std::path::MAIN_SEPARATOR));
                            let sub_results = collect_files(&full_path, &format!("{}  ", prefix))?;
                            results.extend(sub_results);
                        } else {
                            results.push(format!("{}[FILE] {}", prefix, name));
                        }
                    }
                    Ok(results)
                }
                let files = collect_files(Path::new(path), "")?;
                Ok(format!("Recursive contents of '{}':\n{}", path, files.join("\n")))
            }
            Tool::CreateDirectory { path } => {
                fs::create_dir_all(path)?;
                Ok(format!("Directory '{}' created successfully.", path))
            }

            // Search & Navigation
            Tool::GrepSearch { pattern, path } => {
                let search_path = path.as_ref().map(|s| s.as_str()).unwrap_or(".");
                let mut cmd = Command::new("grep");
                cmd.arg("-r").arg("-n").arg("-i").arg(pattern).arg(search_path);
                let output = cmd.output()?;

                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Ok(format!("No matches found for pattern '{}' in '{}'", pattern, search_path))
                }
            }
            Tool::GlobSearch { pattern } => {
                let mut cmd = Command::new("find");
                cmd.arg(".").arg("-name").arg(pattern);
                let output = cmd.output()?;

                if output.status.success() {
                    let stdout_str = String::from_utf8_lossy(&output.stdout);
                    let files_vec: Vec<_> = stdout_str
                        .lines()
                        .filter(|line| !line.is_empty())
                        .collect();
                    Ok(format!("Files matching '{}':\n{}", pattern, files_vec.join("\n")))
                } else {
                    Ok(format!("No files found matching pattern '{}'", pattern))
                }
            }
            Tool::FuzzyFind { pattern, path } => {
                let search_path = path.as_ref().map(|s| s.as_str()).unwrap_or(".");
                let mut cmd = Command::new("find");
                cmd.arg(search_path).arg("-type").arg("f");
                let output = cmd.output()?;

                if output.status.success() {
                    let stdout_str = String::from_utf8_lossy(&output.stdout);
                    let pattern_lower = pattern.to_lowercase();
                    let files_vec: Vec<_> = stdout_str
                        .lines()
                        .filter(|line| !line.is_empty())
                        .filter(|line| line.to_lowercase().contains(&pattern_lower))
                        .collect();
                    if files_vec.is_empty() {
                        Ok(format!("No files found matching fuzzy pattern '{}' in '{}'", pattern, search_path))
                    } else {
                        Ok(format!("Files matching fuzzy pattern '{}':\n{}", pattern, files_vec.join("\n")))
                    }
                } else {
                    Ok(format!("Failed to search files in '{}'", search_path))
                }
            }
            Tool::ReadUrl { url } => {
                let response = reqwest::blocking::get(url)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to fetch URL: {}", e)))?;
                if !response.status().is_success() {
                    return Err(io::Error::new(io::ErrorKind::Other, format!("HTTP Error: {}", response.status())));
                }
                let html = response.text()
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to read response text: {}", e)))?;
                let text = html2text::from_read(html.as_bytes(), 80);
                Ok(format!("Content of {}:\n\n{}", url, text))
            }
            Tool::SearchWeb { query } => {
                // Default to DuckDuckGo HTML scraper for free access
                // In a real production app, we would use a proper API like Tavily or Google Custom Search
                // if configured.
                let provider = web_config.provider.to_lowercase();
                if provider == "duckduckgo" {
                     let url = format!("https://html.duckduckgo.com/html/?q={}", urlencoding::encode(query));
                     let client = reqwest::blocking::Client::builder()
                        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
                        .build()
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                     let response = client.get(&url).send()
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to search: {}", e)))?;

                     if !response.status().is_success() {
                         return Err(io::Error::new(io::ErrorKind::Other, format!("Search failed with status: {}", response.status())));
                     }

                     let html = response.text()
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                     // Simple parsing for DDG HTML (this is brittle but works for basic scraping)
                     // We look for class="result__a"
                     let regex = Regex::new(r#"class="result__a"\s+href="([^"]+)">([^<]+)</a>"#).unwrap();
                     let mut results = Vec::new();
                     for cap in regex.captures_iter(&html).take(5) {
                         let link = &cap[1];
                         let title = &cap[2];
                         // DDG links are often wrapped in /l/?kh=-1&uddg=...
                         let clean_link = if let Some(idx) = link.find("uddg=") {
                             urlencoding::decode(&link[idx+5..]).unwrap_or(std::borrow::Cow::Borrowed(link)).to_string()
                         } else {
                             link.to_string()
                         };
                         results.push(format!("- [{}]({})", title, clean_link));
                     }

                     if results.is_empty() {
                         // Fallback to text conversion if regex fails (DDG layout might have changed)
                         let text = html2text::from_read(html.as_bytes(), 80);
                         Ok(format!("Search results for '{}' (raw text):\n\n{}", query, text.chars().take(1000).collect::<String>()))
                     } else {
                         Ok(format!("Search results for '{}':\n\n{}", query, results.join("\n")))
                     }
                } else {
                    Ok(format!("Provider '{}' not yet implemented. Please use 'duckduckgo'.", provider))
                }
            }

            Tool::Remember { fact } => {
                let memory_file = ".agent_memory.md";
                let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
                let entry = format!("\n## [{}]\n{}\n", timestamp, fact);

                if Path::new(memory_file).exists() {
                    let mut file = fs::OpenOptions::new().append(true).open(memory_file)?;
                    use std::io::Write;
                    file.write_all(entry.as_bytes())?;
                } else {
                    fs::write(memory_file, format!("# Agent Memory\n{}", entry))?;
                }

                Ok(format!("Remembered: '{}' (saved to {})", fact, memory_file))
            }

            Tool::Recall => {
                let memory_file = ".agent_memory.md";
                if Path::new(memory_file).exists() {
                    let content = fs::read_to_string(memory_file)?;
                    Ok(format!("Project Memory:\n\n{}", content))
                } else {
                    Ok("No project memory found. Use REMEMBER to save important facts.".to_string())
                }
            }

            Tool::IndexCodebase { path } => {
                let root_path = Path::new(path);
                if !root_path.exists() {
                    return Err(io::Error::new(io::ErrorKind::NotFound, format!("Path '{}' does not exist", path)));
                }

                let mut index_data: HashMap<String, Vec<String>> = HashMap::new();
                let mut file_count = 0;
                let mut symbol_count = 0;

                // Regex patterns for different languages
                let rust_fn = Regex::new(r"fn\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
                let rust_struct = Regex::new(r"(struct|enum|trait)\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
                let py_def = Regex::new(r"def\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
                let py_class = Regex::new(r"class\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
                let js_func = Regex::new(r"(?:function\s+([a-zA-Z_][a-zA-Z0-9_]*)|([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*function)").unwrap();
                let js_class = Regex::new(r"class\s+([a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
                let js_const = Regex::new(r"(?:const|let|var)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=").unwrap();

                fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&Path)) -> io::Result<()> {
                    if dir.is_dir() {
                        for entry in fs::read_dir(dir)? {
                            let entry = entry?;
                            let path = entry.path();
                            if path.is_dir() {
                                if !path.file_name().unwrap().to_string_lossy().starts_with('.') {
                                     visit_dirs(&path, cb)?;
                                }
                            } else {
                                cb(&path);
                            }
                        }
                    }
                    Ok(())
                }
                visit_dirs(root_path, &mut |file_path| {
                    if let Ok(content) = fs::read_to_string(file_path) {
                        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
                        let file_str = file_path.to_string_lossy().to_string();
                        let mut symbols = Vec::new();
                        match ext {
                            "rs" => {
                                for cap in rust_fn.captures_iter(&content) {
                                    symbols.push(format!("Function: {}", &cap[1]));
                                }
                                for cap in rust_struct.captures_iter(&content) {
                                    symbols.push(format!("{}: {}", &cap[1], &cap[2]));
                                }
                            },
                            "py" => {
                                for cap in py_def.captures_iter(&content) {
                                    symbols.push(format!("Function: {}", &cap[1]));
                                }
                                for cap in py_class.captures_iter(&content) {
                                    symbols.push(format!("Class: {}", &cap[1]));
                                }
                            },
                            "js" | "ts" | "jsx" | "tsx" => {
                                for cap in js_func.captures_iter(&content) {
                                    if let Some(name) = cap.get(1).or(cap.get(2)) {
                                        symbols.push(format!("Function: {}", name.as_str()));
                                    }
                                }
                                for cap in js_class.captures_iter(&content) {
                                    symbols.push(format!("Class: {}", &cap[1]));
                                }
                                for cap in js_const.captures_iter(&content) {
                                    symbols.push(format!("Variable: {}", &cap[1]));
                                }
                            },
                            _ => {}
                        }

                        if !symbols.is_empty() {
                            index_data.insert(file_str, symbols);
                            file_count += 1;
                            symbol_count += index_data.values().last().unwrap().len();
                        }
                    }
                })?;

                // Save index to file
                let json = serde_json::to_string_pretty(&index_data).unwrap();
                fs::write(".agent_index.json", json)?;

                Ok(format!("Indexed {} files and found {} symbols. Index saved to .agent_index.json", file_count, symbol_count))
            }

            Tool::SearchIndex { query } => {
                if !Path::new(".agent_index.json").exists() {
                    return Ok("Index not found. Please run INDEX_CODEBASE first.".to_string());
                }
                let content = fs::read_to_string(".agent_index.json")?;
                let index: HashMap<String, Vec<String>> = serde_json::from_str(&content).unwrap_or_default();
                let query_lower = query.to_lowercase();
                let mut results = Vec::new();
                for (file, symbols) in index {
                    for symbol in symbols {
                        if symbol.to_lowercase().contains(&query_lower) {
                            results.push(format!("{} -> {}", symbol, file));
                        }
                    }
                }

                if results.is_empty() {
                    Ok(format!("No symbols found matching '{}'", query))
                } else {
                    results.sort();
                    Ok(format!("Found {} matches for '{}':\n{}", results.len(), query, results.join("\n")))
                }
            }

            // Code Execution & Compilation
            Tool::ExecuteCode { language, code } => {
                match language.to_lowercase().as_str() {
                    "python" | "py" => Self::execute_python(code),
                    "javascript" | "js" | "node" => Self::execute_javascript(code),
                    "bash" | "sh" => Self::execute_bash(code),
                    "rust" => Self::execute_rust(code),
                    "go" => Self::execute_go(code),
                    "java" => Self::execute_java(code),
                    "c" | "cpp" | "c++" => Self::execute_c_cpp(code, language),
                    _ => Ok(format!("Unsupported language: {}. Supported: python, javascript, bash, rust, go, java, c, cpp", language))
                }
            }
            Tool::RunCommand { command } => {
                // OS-adaptive command execution
                let output = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .arg("/C")
                        .arg(command)
                        .output()?
                } else {
                    Command::new("sh")
                        .arg("-c")
                        .arg(command)
                        .output()?
                };

                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if stdout.is_empty() && stderr.is_empty() {
                        Ok(format!("Command '{}' executed successfully.", command))
                    } else {
                        Ok(format!("Command: {}\n\nSTDOUT:\n{}\n\nSTDERR:\n{}", command, stdout, stderr))
                    }
                } else {
                    Ok(format!("Command '{}' failed:\n\nSTDERR:\n{}", command, String::from_utf8_lossy(&output.stderr)))
                }
            }

            // Development Workflow
            Tool::GitStatus => {
                let output = Command::new("git").arg("status").arg("--porcelain").output()?;
                if output.status.success() {
                    let status = String::from_utf8_lossy(&output.stdout);
                    if status.trim().is_empty() {
                        Ok("Git repository is clean - no changes to commit.".to_string())
                    } else {
                        Ok(format!("Git status:\n{}", status))
                    }
                } else {
                    Ok("Not a git repository or git command failed.".to_string())
                }
            }
            Tool::GitDiff => {
                let output = Command::new("git").arg("diff").output()?;
                if output.status.success() {
                    let diff = String::from_utf8_lossy(&output.stdout);
                    if diff.trim().is_empty() {
                        Ok("No unstaged changes.".to_string())
                    } else {
                        Ok(format!("Git diff:\n{}", diff))
                    }
                } else {
                    Ok("Failed to get git diff.".to_string())
                }
            }
            Tool::GitCommit { message } => {
                // First add all changes
                let _ = Command::new("git").arg("add").arg(".").output()?;
                let output = Command::new("git").arg("commit").arg("-m").arg(message).output()?;
                if output.status.success() {
                    Ok(format!("Successfully committed changes with message: '{}'", message))
                } else {
                    Ok(format!("Commit failed: {}", String::from_utf8_lossy(&output.stderr)))
                }
            }
            Tool::GitLog { count } => {
                let mut cmd = Command::new("git");
                cmd.arg("log").arg("--oneline");
                if let Some(n) = count {
                    cmd.arg(format!("-{}", n));
                }
                let output = cmd.output()?;
                if output.status.success() {
                    Ok(format!("Git log:\n{}", String::from_utf8_lossy(&output.stdout)))
                } else {
                    Ok("Failed to get git log.".to_string())
                }
            }

            // Quality Assurance
            Tool::RunLint { language, path } => {
                let target_path = path.as_ref().map(|s| s.as_str()).unwrap_or(".");
                match language.to_lowercase().as_str() {
                    "rust" => {
                        let mut cmd = Command::new("cargo");
                        cmd.arg("clippy").current_dir(target_path);
                        let output = cmd.output()?;
                        if output.status.success() {
                            Ok("Cargo clippy passed - no linting issues found.".to_string())
                        } else {
                            Ok(format!("Linting issues found:\n{}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "python" => {
                        let output = if let Ok(out) = Command::new("flake8").arg(target_path).output() {
                            out
                        } else if let Ok(out) = Command::new("pylint").arg(target_path).output() {
                            out
                        } else {
                            match Command::new("python").arg("-m").arg("pycodestyle").arg(target_path).output() {
                                Ok(out) => out,
                                Err(_) => std::process::Output {
                                    status: std::process::ExitStatus::default(),
                                    stdout: Vec::new(),
                                    stderr: b"Python linter not available".to_vec(),
                                }
                            }
                        };
                        if output.status.success() {
                            Ok("Python linting passed.".to_string())
                        } else {
                            Ok(format!("Python linting issues:\n{}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "javascript" | "js" => {
                        let output = match Command::new("eslint").arg(target_path).output() {
                            Ok(out) => out,
                            Err(_) => std::process::Output {
                                status: std::process::ExitStatus::default(),
                                stdout: Vec::new(),
                                stderr: b"ESLint not available".to_vec(),
                            }
                        };
                        if output.status.success() {
                            Ok("JavaScript linting passed.".to_string())
                        } else {
                            Ok(format!("JavaScript linting issues:\n{}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    _ => Ok(format!("Linting not supported for language: {}", language))
                }
            }
            Tool::RunTests { framework, path } => {
                let target_path = path.as_ref().map(|s| s.as_str()).unwrap_or(".");
                match framework.to_lowercase().as_str() {
                    "cargo" |                     "rust" => {
                        let mut cmd = Command::new("cargo");
                        cmd.arg("test").current_dir(target_path);
                        let output = cmd.output()?;
                        if output.status.success() {
                            Ok(format!("Tests passed!\n{}", String::from_utf8_lossy(&output.stdout)))
                        } else {
                            Ok(format!("Tests failed:\n{}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "pytest" | "python" => {
                        let mut cmd = Command::new("python");
                        cmd.arg("-m").arg("pytest").current_dir(target_path);
                        let output = if let Ok(out) = cmd.output() {
                            out
                        } else {
                            let mut cmd2 = Command::new("python");
                            cmd2.arg("-m").arg("unittest").arg("discover").current_dir(target_path);
                            match cmd2.output() {
                                Ok(out) => out,
                                Err(_) => std::process::Output {
                                    status: std::process::ExitStatus::default(),
                                    stdout: Vec::new(),
                                    stderr: b"Python testing framework not available".to_vec(),
                                }
                            }
                        };
                        if output.status.success() {
                            Ok(format!("Python tests passed!\n{}", String::from_utf8_lossy(&output.stdout)))
                        } else {
                            Ok(format!("Python tests failed:\n{}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "jest" | "javascript" | "js" => {
                        let mut cmd = Command::new("npx");
                        cmd.arg("jest").current_dir(target_path);
                        let output = match cmd.output() {
                            Ok(out) => out,
                            Err(_) => std::process::Output {
                                status: std::process::ExitStatus::default(),
                                stdout: Vec::new(),
                                stderr: b"Jest testing framework not available".to_vec(),
                            }
                        };
                        if output.status.success() {
                            Ok(format!("JavaScript tests passed!\n{}", String::from_utf8_lossy(&output.stdout)))
                        } else {
                            Ok(format!("JavaScript tests failed:\n{}", String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    _ => Ok(format!("Testing framework '{}' not supported.", framework))
                }
            }

            // Package Management
            Tool::InstallPackage { manager, package } => {
                match manager.to_lowercase().as_str() {
                    "npm" | "yarn" => {
                        let cmd = if manager == "yarn" { "yarn" } else { "npm" };
                        let output = Command::new(cmd).arg("install").arg(package).output()?;
                        if output.status.success() {
                            Ok(format!("Successfully installed {} package: {}", manager, package))
                        } else {
                            Ok(format!("Failed to install {} package '{}': {}", manager, package, String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "cargo" | "rust" => {
                        let output = Command::new("cargo").arg("add").arg(package).output()?;
                        if output.status.success() {
                            Ok(format!("Successfully added Rust crate: {}", package))
                        } else {
                            Ok(format!("Failed to add Rust crate '{}': {}", package, String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "pip" | "python" => {
                        let output = Command::new("pip").arg("install").arg(package).output()?;
                        if output.status.success() {
                            Ok(format!("Successfully installed Python package: {}", package))
                        } else {
                            Ok(format!("Failed to install Python package '{}': {}", package, String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    "go" => {
                        let output = Command::new("go").arg("get").arg(package).output()?;
                        if output.status.success() {
                            Ok(format!("Successfully installed Go package: {}", package))
                        } else {
                            Ok(format!("Failed to install Go package '{}': {}", package, String::from_utf8_lossy(&output.stderr)))
                        }
                    }
                    _ => Ok(format!("Package manager '{}' not supported.", manager))
                }
            }

            // Planning and Task Management
            Tool::CreatePlan { task, steps } => {
                let mut content = format!("# Task Plan: {}\n\n", task);
                content.push_str("## Checklist:\n\n");
                for (i, step) in steps.iter().enumerate() {
                    content.push_str(&format!("- [ ] Step {}: {}\n", i + 1, step));
                }
                content.push_str("\n## Progress:\n");
                content.push_str("- Total Steps: 0\n");
                content.push_str("- Completed: 0\n");
                content.push_str("- Remaining: 0\n");

                fs::write("plan.md", content)?;
                Ok(format!("Created plan.md with {} steps for task: {}", steps.len(), task))
            }

            Tool::UpdatePlan { completed_step } => {
                let content = fs::read_to_string("plan.md")?;
                let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

                // Update the checklist
                for line in &mut lines {
                    if line.contains(&format!("Step {}:", completed_step)) {
                        *line = line.replace("- [ ]", "- [x]");
                    }
                }

                // Update progress section
                let total_steps = lines.iter().filter(|line| line.contains("Step ")).count();
                let completed_steps = lines.iter().filter(|line| line.contains("- [x]")).count();

                for line in &mut lines {
                    if line.contains("Total Steps:") {
                        *line = format!("- Total Steps: {}", total_steps);
                    } else if line.contains("Completed:") {
                        *line = format!("- Completed: {}", completed_steps);
                    } else if line.contains("Remaining:") {
                        *line = format!("- Remaining: {}", total_steps.saturating_sub(completed_steps));
                    }
                }

                let new_content = lines.join("\n");
                fs::write("plan.md", new_content)?;
                Ok(format!("Updated plan.md: Step {} completed. Progress: {}/{}", completed_step, completed_steps, total_steps))
            }

            Tool::ClearPlan => {
                if Path::new("plan.md").exists() {
                    fs::remove_file("plan.md")?;
                    Ok("Cleared plan.md - task completed!".to_string())
                } else {
                    Ok("No plan.md file found to clear".to_string())
                }
            }

            // System Information & Time
            Tool::GetTime => {
                use std::time::SystemTime;
                let now = SystemTime::now();
                let datetime = chrono::Local::now();
                let unix_timestamp = now.duration_since(SystemTime::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("System time is before UNIX epoch: {}", e)))?;
                Ok(format!(
                    "Current Date & Time:\n\
                     • Date: {}\n\
                     • Time: {}\n\
                     • Timezone: {}\n\
                     • Unix Timestamp: {}",
                    datetime.format("%Y-%m-%d"),
                    datetime.format("%H:%M:%S"),
                    datetime.format("%Z"),
                    unix_timestamp
                ))
            }

            Tool::GetOsInfo => {
                let os = env::consts::OS;
                let arch = env::consts::ARCH;
                let family = env::consts::FAMILY;

                // Detect specific OS details
                let os_details = match os {
                    "linux" => {
                        // Try to get distribution info
                        let distro = fs::read_to_string("/etc/os-release")
                            .ok()
                            .and_then(|content| {
                                content.lines()
                                    .find(|line| line.starts_with("PRETTY_NAME="))
                                    .map(|line| line.trim_start_matches("PRETTY_NAME=").trim_matches('"').to_string())
                            })
                            .unwrap_or_else(|| "Unknown Linux Distribution".to_string());
                        format!("Linux ({})", distro)
                    }
                    "macos" => "macOS".to_string(),
                    "windows" => "Windows".to_string(),
                    other => other.to_string(),
                };

                // Get shell command separator based on OS
                let shell_type = if os == "windows" { "cmd.exe / PowerShell" } else { "bash / sh" };
                let path_sep = std::path::MAIN_SEPARATOR;
                let cmd_sep = if os == "windows" { "&" } else { "&&" };

                Ok(format!(
                    "Operating System Information:\n\
                     • OS: {}\n\
                     • Architecture: {}\n\
                     • OS Family: {}\n\
                     • Shell: {}\n\
                     • Path Separator: {}\n\
                     • Command Separator: {}\n\
                     • Temp Directory: {}",
                    os_details,
                    arch,
                    family,
                    shell_type,
                    path_sep,
                    cmd_sep,
                    env::temp_dir().display()
                ))
            }

            // Enhanced File Operations
            Tool::CopyFile { source, destination } => {
                fs::copy(source, destination)?;
                Ok(format!("Successfully copied '{}' to '{}'", source, destination))
            }

            Tool::MoveFile { source, destination } => {
                fs::rename(source, destination)?;
                Ok(format!("Successfully moved '{}' to '{}'", source, destination))
            }

            Tool::RenameFile { old_name, new_name } => {
                fs::rename(old_name, new_name)?;
                Ok(format!("Successfully renamed '{}' to '{}'", old_name, new_name))
            }
        }
    }

    // Helper methods for code execution
    fn execute_python(code: &str) -> Result<String, io::Error> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let temp_file = format!("/tmp/temp_code_{}.py", timestamp);
        fs::write(&temp_file, code)?;
        let output = Command::new("python3").arg(&temp_file).output()?;
        let _ = fs::remove_file(temp_file);
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Ok(format!(
                "Python execution failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    fn execute_javascript(code: &str) -> Result<String, io::Error> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let temp_file = format!("/tmp/temp_code_{}.js", timestamp);
        fs::write(&temp_file, code)?;
        let output = Command::new("node").arg(&temp_file).output()?;
        let _ = fs::remove_file(temp_file);
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Ok(format!(
                "JavaScript execution failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    fn execute_bash(code: &str) -> Result<String, io::Error> {
        let output = Command::new("bash").arg("-c").arg(code).output()?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Ok(format!(
                "Bash execution failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    fn execute_rust(code: &str) -> Result<String, io::Error> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let temp_dir = format!("/tmp/rust_code_{}", timestamp);
        fs::create_dir_all(&temp_dir)?;
        let main_rs = format!("{}/src/main.rs", temp_dir);
        let cargo_toml = format!("{}/Cargo.toml", temp_dir);

        fs::write(
            &cargo_toml,
            r#"[package]
name = "temp_code"
version = "0.1.0"
edition = "2021"
[dependencies]
"#,
        )?;
        fs::create_dir_all(format!("{}/src", temp_dir))?;
        fs::write(&main_rs, code)?;
        let mut cmd = Command::new("cargo");
        cmd.arg("run").current_dir(&temp_dir);
        let output = cmd.output()?;
        let _ = fs::remove_dir_all(temp_dir);
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Ok(format!(
                "Rust execution failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    fn execute_go(code: &str) -> Result<String, io::Error> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let temp_file = format!("/tmp/temp_code_{}.go", timestamp);
        fs::write(&temp_file, format!("package main\n\n{}", code))?;
        let output = Command::new("go").arg("run").arg(&temp_file).output()?;
        let _ = fs::remove_file(temp_file);
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Ok(format!(
                "Go execution failed:\n{}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    fn execute_java(code: &str) -> Result<String, io::Error> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let temp_file = format!("/tmp/temp_code_{}.java", timestamp);
        let class_name = "TempCode";
        let full_code = format!("public class {} {{\n    public static void main(String[] args) {{\n        {}\n    }}\n}}", class_name, code);
        fs::write(&temp_file, full_code)?;
        let compile_output = Command::new("javac").arg(&temp_file).output()?;
        if compile_output.status.success() {
            let run_output = Command::new("java")
                .arg("-cp")
                .arg("/tmp")
                .arg(class_name)
                .output()?;
            let _ = fs::remove_file(temp_file);
            let _ = fs::remove_file(format!("/tmp/{}.class", class_name));
            if run_output.status.success() {
                Ok(String::from_utf8_lossy(&run_output.stdout).to_string())
            } else {
                Ok(format!(
                    "Java execution failed:\n{}",
                    String::from_utf8_lossy(&run_output.stderr)
                ))
            }
        } else {
            let _ = fs::remove_file(temp_file);
            Ok(format!(
                "Java compilation failed:\n{}",
                String::from_utf8_lossy(&compile_output.stderr)
            ))
        }
    }

    fn execute_c_cpp(code: &str, language: &str) -> Result<String, io::Error> {
        let is_cpp = matches!(language.to_lowercase().as_str(), "cpp" | "c++");
        let extension = if is_cpp { "cpp" } else { "c" };
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let temp_source = format!("/tmp/temp_code_{}.{}", timestamp, extension);
        let temp_exe = format!("/tmp/temp_exe_{}", timestamp);

        let full_code = if is_cpp {
            format!(
                "#include <iostream>\nint main() {{\n{}\nreturn 0;\n}}",
                code
            )
        } else {
            format!("#include <stdio.h>\nint main() {{\n{}\nreturn 0;\n}}", code)
        };

        fs::write(&temp_source, full_code)?;
        let compiler = if is_cpp { "g++" } else { "gcc" };
        let compile_output = Command::new(compiler)
            .arg(&temp_source)
            .arg("-o")
            .arg(&temp_exe)
            .output()?;
        if compile_output.status.success() {
            let run_output = Command::new(&temp_exe).output()?;
            let _ = fs::remove_file(temp_source);
            let _ = fs::remove_file(temp_exe);
            if run_output.status.success() {
                Ok(String::from_utf8_lossy(&run_output.stdout).to_string())
            } else {
                Ok(format!(
                    "{} execution failed:\n{}",
                    if is_cpp { "C++" } else { "C" },
                    String::from_utf8_lossy(&run_output.stderr)
                ))
            }
        } else {
            let _ = fs::remove_file(temp_source);
            Ok(format!(
                "{} compilation failed:\n{}",
                if is_cpp { "C++" } else { "C" },
                String::from_utf8_lossy(&compile_output.stderr)
            ))
        }
    }
}

#[derive(Clone)]
pub struct Agent {
    messages: Vec<Message>,
}

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
}

impl Agent {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    fn get_system_prompt() -> String {
        // Load custom prompt if it exists
        let custom_prompt = fs::read_to_string("prompt.md")
            .ok()
            .map(|content| format!("\n\n## CUSTOM USER INSTRUCTIONS\n\n{}\n", content))
            .unwrap_or_default();

        format!(
            r#"You are an advanced AI coding assistant with comprehensive access to development tools. You excel at software development, debugging, and project management. You MUST use tools to complete tasks - never just describe what you would do.

## ReAct PATTERN: REASON -> ACT -> OBSERVE

**You MUST follow the ReAct (Reasoning + Acting) pattern for all tasks:**

1. **REASON**: Before acting, explicitly think through what needs to be done
2. **ACT**: Execute the appropriate tool to accomplish the task
3. **OBSERVE**: Analyze the tool's output and decide next steps

**Example ReAct Pattern:**
```
REASONING: I need to understand the project structure before making changes. Let me list the files first.

TOOL: {{"name": "LIST_FILES", "parameters": {{"path": "."}}}}
```

The tool will execute automatically and you will receive the result. Then you can reason about the result and take the next action.

**CRITICAL FIRST STEP: For complex tasks requiring 3+ steps or multiple components, ALWAYS start with CREATE_PLAN as your very first tool call. Do not execute any other tools until the plan is created.**

## AVAILABLE TOOLS

### Planning and Task Management
1. **CREATE_PLAN** `<task> <steps>` - Create a structured plan in plan.md breaking down tasks into steps
2. **UPDATE_PLAN** `<step_number>` - Mark a specific step as completed in plan.md
3. **CLEAR_PLAN** - Remove plan.md when task is fully completed

### File Operations
4. **READ_FILE** `<path>` - Read and display file contents with line numbers
5. **WRITE_FILE** `<path> <content>` - Create or overwrite files (creates parent directories automatically)
6. **APPEND_FILE** `<path> <content>` - Add content to the end of existing files
7. **SEARCH_REPLACE** `<path> <old_string> <new_string>` - Replace text in files with exact matching
8. **DELETE_FILE** `<path>` - Remove files or directories (recursive for directories)

### Directory Operations
9. **LIST_FILES** `<path>` - List files and directories in a given path
10. **LIST_FILES_RECURSIVE** `<path>` - Recursively list all files in directory tree
11. **CREATE_DIRECTORY** `<path>` - Create directories (recursive)

### Search & Navigation
12. **GREP_SEARCH** `<pattern> [path]` - Search for text patterns using ripgrep (fast, regex support, case-insensitive)
13. **GLOB_SEARCH** `<pattern>` - Find files matching glob patterns (*.rs, **/test/**, etc.)
14. **FUZZY_FIND** `<pattern> [path]` - Fuzzy search for file paths (e.g. "user" matches "src/user_model.rs")
15. **INDEX_CODEBASE** `<path>` - Scan directory and build a symbol index (functions, classes)
16. **SEARCH_INDEX** `<query>` - Search the built index for symbols
17. **READ_URL** `<url>` - Fetch and read the content of a web page
18. **SEARCH_WEB** `<query>` - Search the web for information (default: DuckDuckGo)
19. **REMEMBER** `<fact>` - Save an important fact or decision to project memory (.agent_memory.md)
20. **RECALL** - Retrieve all saved project memory

### Code Execution & Compilation
21. **EXECUTE_CODE** `<language> <code>` - Execute code in multiple languages:
    - Python (python, py)
    - JavaScript/Node.js (javascript, js, node)
    - Bash/Shell (bash, sh)
    - Rust (rust)
    - Go (go)
    - Java (java)
    - C/C++ (c, cpp, c++)
22. **RUN_COMMAND** `<command>` - Execute shell commands with full environment access

### Development Workflow
23. **GIT_STATUS** - Show git repository status
24. **GIT_DIFF** - Show unstaged changes
25. **GIT_COMMIT** `<message>` - Commit changes with message
26. **GIT_LOG** `[count]` - Show recent commit history

### Quality Assurance
27. **RUN_LINT** `<language> [path]` - Run linters for code quality:
    - Rust: cargo clippy
    - Python: flake8/pylint
    - JavaScript: eslint
    - Go: golangci-lint
28. **RUN_TESTS** `<framework> [path]` - Run test suites:
    - Rust: cargo test
    - Python: pytest/unittest
    - JavaScript: jest/mocha
    - Go: go test

### Package Management
29. **INSTALL_PACKAGE** `<manager> <package>` - Install packages:
    - npm install <package>
    - cargo add <package>
    - pip install <package>
    - go get <package>

### System Information
30. **GET_TIME** - Get current date, time, and timezone information from the system
31. **GET_OS_INFO** - Get operating system details (OS type, architecture, shell, path separators)

### Enhanced File Operations
32. **COPY_FILE** `<source> <destination>` - Copy a file from source to destination
33. **MOVE_FILE** `<source> <destination>` - Move/relocate a file or directory
34. **RENAME_FILE** `<old_name> <new_name>` - Rename a file or directory

## PLANNING WORKFLOW - REQUIRED FOR COMPLEX TASKS

**MANDATORY PLANNING REQUIREMENT:**

**ALWAYS USE CREATE_PLAN for tasks that require 3+ steps or involve multiple files/components.**

**Examples of tasks that REQUIRE planning:**
- Building a web application (frontend + backend)
- Creating a new project from scratch
- Implementing authentication systems
- Setting up CI/CD pipelines
- Refactoring large codebases
- Adding new features with multiple components
- Database design and implementation
- API development with multiple endpoints

**Examples of tasks that DON'T require planning:**
- Reading a single file
- Simple text search
- Running a single command
- Making a small code change in one file

**PLANNING PROCESS - ALWAYS FOLLOW THIS EXACT SEQUENCE:**

1. **FIRST RESPONSE**: Always start with CREATE_PLAN for complex tasks
   ```
   TOOL: {{"name": "CREATE_PLAN", "parameters": {{"task": "Build a React todo app", "steps": ["Set up React project", "Create components", "Implement state management", "Add styling", "Test functionality"]}}}}
   ```

2. **EXECUTE STEPS**: Complete each step one by one
   ```
   TOOL: {{"name": "UPDATE_PLAN", "parameters": {{"completed_step": 1}}}}
   ```

3. **FINAL STEP**: Clean up when done
   ```
   TOOL: {{"name": "CLEAR_PLAN", "parameters": {{}}}}
   ```

**DECISION TREE FOR TASK HANDLING:**

```
User Request Received
├── Is it a simple task? (1-2 steps, single file)
│   ├── YES -> Use tools directly (READ_FILE, WRITE_FILE, etc.)
│   └── NO -> Continue to planning
└── Is it a complex task? (3+ steps, multiple files/components)
    ├── YES -> CREATE_PLAN as FIRST tool call
    └── NO -> Use tools directly

EXAMPLES:
Simple: "Read the main.rs file" -> READ_FILE
Simple: "Change function name" -> SEARCH_REPLACE
Complex: "Build a web app" -> CREATE_PLAN first
Complex: "Add authentication" -> CREATE_PLAN first
```

**CRITICAL: If you receive a complex task, your FIRST tool call MUST be CREATE_PLAN. Do not execute any other tools until the plan is created.**

## TOOL CALLING FORMAT

**PRIMARY FORMAT: Use JSON tool calls for maximum reliability:**
```json
TOOL: {{"name": "READ_FILE", "parameters": {{"path": "/path/to/file.txt"}}}}
TOOL: {{"name": "CREATE_PLAN", "parameters": {{"task": "Task description", "steps": ["Step 1", "Step 2", "Step 3"]}}}}
```

**Legacy format (still supported):**
```
TOOL: READ_FILE /path/to/file.txt
TOOL: CREATE_PLAN "Task description" "Step 1" "Step 2" "Step 3"
```

**CRITICAL RULES FOR TOOL EXECUTION:**
1. Put ONLY the tool call on a line starting with "TOOL:"
2. Do NOT add "ACTION:" or any prefix before "TOOL:"
3. Do NOT show placeholder text like "[Tool executes and returns result]"
4. The tool will execute automatically - you don't need to describe the execution
5. After the tool executes, you will receive the result and can reason about it

**CORRECT:**
```
TOOL: {{"name": "GET_TIME", "parameters": {{}}}}
```

**INCORRECT:**
```
ACTION: TOOL: {{"name": "GET_TIME", "parameters": {{}}}}
```

**INCORRECT:**
```
TOOL: {{"name": "GET_TIME", "parameters": {{}}}}
[Tool executes and returns result]
```

## CODING WORKFLOW PRINCIPLES

### 1. Task Assessment Phase
- **Determine task complexity**: If task requires 3+ steps or multiple files -> USE CREATE_PLAN
- **Assess scope**: Single file changes = direct tools, multi-component tasks = planning required

### 2. Exploration Phase
- Always start with LIST_FILES to understand project structure
- Use GLOB_SEARCH to find relevant files (*.rs for Rust, *.py for Python, etc.)
- Use FUZZY_FIND to locate files by partial name (e.g. "user" -> "src/user_model.rs")
- READ_FILE key configuration files (Cargo.toml, package.json, requirements.txt, etc.)

### 3. Code Understanding Phase
- Use GREP_SEARCH to find code definitions, references, or TODOs
- Use FUZZY_FIND to jump to specific files
- READ_FILE to examine the code context and logic
- Analyze the code structure before making changes

### 4. Planning Phase (MANDATORY for complex tasks)
- **FIRST STEP**: Use CREATE_PLAN to break down the task
- Break complex tasks into manageable steps
- Identify dependencies and prerequisites
- Plan file modifications before executing

### 5. Implementation Phase
- Use SEARCH_REPLACE for precise edits (prefer over WRITE_FILE for modifications)
- APPEND_FILE for adding to existing files
- Verify changes with READ_FILE
- Test modifications with EXECUTE_CODE or RUN_COMMAND

### 6. Verification Phase
- Use RUN_LINT to check code quality
- Execute tests with RUN_TESTS
- Build/compile with RUN_COMMAND
- Verify functionality with EXECUTE_CODE

### 7. Completion Phase
- Use UPDATE_PLAN to mark steps as completed
- Use CLEAR_PLAN when all steps are done

### 8. Error Recovery
- If SEARCH_REPLACE fails, check exact string matching
- If EXECUTE_CODE fails, try RUN_COMMAND with compilation
- If RUN_COMMAND fails, simplify the command or check permissions
- Always verify file existence with LIST_FILES before operations

## EXAMPLES

### Planning:
TOOL: {{"name": "CREATE_PLAN", "parameters": {{"task": "Build a React component", "steps": ["Create component file", "Add state management", "Implement event handlers", "Add styling", "Test component"]}}}}
TOOL: {{"name": "UPDATE_PLAN", "parameters": {{"completed_step": 1}}}}
TOOL: {{"name": "CLEAR_PLAN", "parameters": {{}}}}

### File Operations:
TOOL: {{"name": "READ_FILE", "parameters": {{"path": "src/main.rs"}}}}
TOOL: {{"name": "WRITE_FILE", "parameters": {{"path": "hello.txt", "content": "Hello World!"}}}}
TOOL: {{"name": "SEARCH_REPLACE", "parameters": {{"path": "src/main.rs", "old_string": "fn main() {{", "new_string": "fn main() {{\n    println!(\"Hello!\");"}}}}

### Development Tasks:
TOOL: {{"name": "LIST_FILES", "parameters": {{"path": "."}}}}
TOOL: {{"name": "GREP_SEARCH", "parameters": {{"pattern": "TODO|FIXME", "path": "src/"}}}}
TOOL: {{"name": "FUZZY_FIND", "parameters": {{"pattern": "main", "path": "src/"}}}}
TOOL: {{"name": "EXECUTE_CODE", "parameters": {{"language": "rust", "code": "fn main() {{ println!(\"test\"); }}"}}}}
TOOL: {{"name": "RUN_LINT", "parameters": {{"language": "rust"}}}}

### Quality Assurance:
TOOL: {{"name": "RUN_TESTS", "parameters": {{"framework": "cargo"}}}}
TOOL: {{"name": "GIT_STATUS", "parameters": {{}}}}
TOOL: {{"name": "RUN_COMMAND", "parameters": {{"command": "cargo build --release"}}}}

### System Information & File Management:
TOOL: {{"name": "GET_TIME", "parameters": {{}}}}
TOOL: {{"name": "GET_OS_INFO", "parameters": {{}}}}
TOOL: {{"name": "COPY_FILE", "parameters": {{"source": "file.txt", "destination": "backup/file.txt"}}}}
TOOL: {{"name": "MOVE_FILE", "parameters": {{"source": "old/path/file.txt", "destination": "new/path/file.txt"}}}}
TOOL: {{"name": "RENAME_FILE", "parameters": {{"old_name": "oldname.txt", "new_name": "newname.txt"}}}}

## OS-ADAPTIVE EXECUTION

**The system automatically detects the operating system and adapts commands accordingly:**

- **Windows**: Uses cmd.exe/PowerShell, backslash paths, different command syntax
- **Linux**: Uses bash/sh, forward slash paths, Unix commands
- **macOS**: Similar to Linux with some macOS-specific tools

**Use GET_OS_INFO to check the current environment before executing OS-specific commands.**

**Examples of OS-adaptive commands:**
```
# First check the OS
TOOL: {{"name": "GET_OS_INFO", "parameters": {{}}}}

# Then adapt commands based on OS
# Windows: dir, copy, move, del
# Linux/Mac: ls, cp, mv, rm
```


## CRITICAL GUIDELINES

- **PLAN FIRST for complex tasks** - Always use CREATE_PLAN as your FIRST tool call for tasks requiring 3+ steps
- **ALWAYS use tools** - Never describe actions without executing them
- **Use JSON format** - Prefer `TOOL: {{"name": "TOOL_NAME", "parameters": {{...}}}}` over legacy format
- **Execute immediately** - Do not explain what you would do, just call the tool
- **WORK UNTIL COMPLETE** - After each tool result, immediately determine the next action and execute it. Continue this loop until the user's request is fully satisfied. Do NOT stop after just one tool call.
- **No placeholders** - Never write text like "[Tool executes]" or "[Result will appear]"
- **Verify work** - Read files after writing, list directories after changes
- **Handle errors gracefully** - Try alternative approaches when tools fail
- **Be precise** - Use exact string matching for SEARCH_REPLACE
- **Think systematically** - Plan before acting, verify after completion
- **Code quality matters** - Use linters and tests to maintain standards
- **Use ReAct pattern** - REASON (think about what to do next) → ACT (execute tool) → OBSERVE (check result) → REPEAT until task is complete
- **Check OS compatibility** - Use GET_OS_INFO when executing OS-specific commands
- **AUTONOMOUS EXECUTION** - You must ALWAYS execute tools. Never ask the user for permission to run a tool unless explicitly told to do so. Keep executing tools in sequence until the user's request is fully complete.
- **Leverage time awareness** - Use GET_TIME when timestamps or scheduling matters

**REMEMBER:
1. For complex tasks, your FIRST response MUST contain CREATE_PLAN. For simple tasks, use tools directly.
2. Your responses should contain actual tool calls that will be executed, not descriptions of tool usage.
3. MOST IMPORTANTLY: After receiving a tool result, IMMEDIATELY proceed with the next step. Do not wait for user input. Continue working through the task until it is COMPLETE.
**{}
"#,
            custom_prompt
        )
    }

    fn parse_tool_call(&self, response: &str) -> Option<Tool> {
        let lines: Vec<&str> = response.lines().collect();
        for line in lines {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with("TOOL:") {
                let tool_part = trimmed_line.strip_prefix("TOOL:").unwrap().trim();

                // Try JSON format first
                if tool_part.starts_with('{') {
                    if let Ok(tool_call) = serde_json::from_str::<ToolCall>(tool_part) {
                        if let Some(tool) = tool_call.into_tool() {
                            return Some(tool);
                        }
                    }
                }

                // Fall back to legacy format
                let parts: Vec<&str> = tool_part.splitn(2, ' ').collect();
                if parts.len() >= 2 {
                    let tool_name = parts[0];
                    let params = parts[1];

                    return match tool_name {
                        "READ_FILE" => Some(Tool::ReadFile {
                            path: params.to_string(),
                        }),
                        "WRITE_FILE" => {
                            if let Some(space_pos) = params.find(' ') {
                                let path = &params[..space_pos];
                                let content = &params[space_pos + 1..];
                                Some(Tool::WriteFile {
                                    path: path.to_string(),
                                    content: content.to_string(),
                                })
                            } else {
                                None
                            }
                        }
                        "APPEND_FILE" => {
                            if let Some(space_pos) = params.find(' ') {
                                let path = &params[..space_pos];
                                let content = &params[space_pos + 1..];
                                Some(Tool::AppendFile {
                                    path: path.to_string(),
                                    content: content.to_string(),
                                })
                            } else {
                                None
                            }
                        }
                        "SEARCH_REPLACE" => {
                            let params_str = params.to_string();
                            // Parse: "path old_string new_string" - need to split carefully
                            if let Some(first_space) = params_str.find(' ') {
                                let path = &params_str[..first_space];
                                let remaining = &params_str[first_space + 1..];
                                // Find the boundary between old_string and new_string
                                // This is tricky with the legacy format, but we'll assume old_string comes first
                                if let Some(last_space) = remaining.rfind(' ') {
                                    let old_string = &remaining[..last_space];
                                    let new_string = &remaining[last_space + 1..];
                                    Some(Tool::SearchReplace {
                                        path: path.to_string(),
                                        old_string: old_string.to_string(),
                                        new_string: new_string.to_string(),
                                    })
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }
                        "RUN_COMMAND" => Some(Tool::RunCommand {
                            command: params.to_string(),
                        }),
                        "LIST_FILES" => Some(Tool::ListFiles {
                            path: params.to_string(),
                        }),
                        "LIST_FILES_RECURSIVE" => Some(Tool::ListFilesRecursive {
                            path: params.to_string(),
                        }),
                        "CREATE_DIRECTORY" => Some(Tool::CreateDirectory {
                            path: params.to_string(),
                        }),
                        "DELETE_FILE" => Some(Tool::DeleteFile {
                            path: params.to_string(),
                        }),
                        "EXECUTE_CODE" => {
                            if let Some(space_pos) = params.find(' ') {
                                let language = &params[..space_pos];
                                let code = &params[space_pos + 1..];
                                Some(Tool::ExecuteCode {
                                    language: language.to_string(),
                                    code: code.to_string(),
                                })
                            } else {
                                None
                            }
                        }
                        "GREP_SEARCH" => {
                            // Parse: "pattern [path]"
                            let search_parts: Vec<&str> = params.splitn(2, ' ').collect();
                            let pattern = search_parts[0].to_string();
                            let path = search_parts.get(1).map(|s| s.to_string());
                            Some(Tool::GrepSearch { pattern, path })
                        }
                        "GLOB_SEARCH" => Some(Tool::GlobSearch {
                            pattern: params.to_string(),
                        }),
                        "FUZZY_FIND" => {
                            let parts: Vec<&str> = params.splitn(2, ' ').collect();
                            let pattern = parts[0].to_string();
                            let path = parts.get(1).map(|s| s.to_string());
                            Some(Tool::FuzzyFind { pattern, path })
                        }
                        "INDEX_CODEBASE" => Some(Tool::IndexCodebase {
                            path: params.to_string(),
                        }),
                        "SEARCH_INDEX" => Some(Tool::SearchIndex {
                            query: params.to_string(),
                        }),
                        "READ_URL" => Some(Tool::ReadUrl {
                            url: params.to_string(),
                        }),
                        "SEARCH_WEB" => Some(Tool::SearchWeb {
                            query: params.to_string(),
                        }),
                        "REMEMBER" => Some(Tool::Remember {
                            fact: params.to_string(),
                        }),
                        "RECALL" => Some(Tool::Recall),
                        "GIT_STATUS" => Some(Tool::GitStatus),
                        "GIT_DIFF" => Some(Tool::GitDiff),
                        "GIT_COMMIT" => Some(Tool::GitCommit {
                            message: params.to_string(),
                        }),
                        "GIT_LOG" => {
                            let count = if params.is_empty() {
                                None
                            } else {
                                params.parse::<usize>().ok()
                            };
                            Some(Tool::GitLog { count })
                        }
                        "RUN_LINT" => {
                            let lint_parts: Vec<&str> = params.splitn(2, ' ').collect();
                            let language = lint_parts[0].to_string();
                            let path = lint_parts.get(1).map(|s| s.to_string());
                            Some(Tool::RunLint { language, path })
                        }
                        "RUN_TESTS" => {
                            let test_parts: Vec<&str> = params.splitn(2, ' ').collect();
                            let framework = test_parts[0].to_string();
                            let path = test_parts.get(1).map(|s| s.to_string());
                            Some(Tool::RunTests { framework, path })
                        }
                        "INSTALL_PACKAGE" => {
                            let package_parts: Vec<&str> = params.splitn(2, ' ').collect();
                            if package_parts.len() == 2 {
                                Some(Tool::InstallPackage {
                                    manager: package_parts[0].to_string(),
                                    package: package_parts[1].to_string(),
                                })
                            } else {
                                None
                            }
                        }
                        "CREATE_PLAN" => {
                            // Parse: "task description" "step1" "step2" "step3"
                            let parts: Vec<&str> = params
                                .split('"')
                                .filter(|s| !s.is_empty() && !s.trim().is_empty())
                                .collect();
                            if parts.len() >= 2 {
                                let task = parts[0].trim().to_string();
                                let steps: Vec<String> =
                                    parts[1..].iter().map(|s| s.trim().to_string()).collect();
                                Some(Tool::CreatePlan { task, steps })
                            } else {
                                None
                            }
                        }
                        "UPDATE_PLAN" => {
                            if let Ok(step) = params.trim().parse::<usize>() {
                                Some(Tool::UpdatePlan {
                                    completed_step: step,
                                })
                            } else {
                                None
                            }
                        }
                        "CLEAR_PLAN" => Some(Tool::ClearPlan),
                        "GET_TIME" => Some(Tool::GetTime),
                        "GET_OS_INFO" => Some(Tool::GetOsInfo),
                        "COPY_FILE" => {
                            let parts: Vec<&str> = params.splitn(2, ' ').collect();
                            if parts.len() == 2 {
                                Some(Tool::CopyFile {
                                    source: parts[0].to_string(),
                                    destination: parts[1].to_string(),
                                })
                            } else {
                                None
                            }
                        }
                        "MOVE_FILE" => {
                            let parts: Vec<&str> = params.splitn(2, ' ').collect();
                            if parts.len() == 2 {
                                Some(Tool::MoveFile {
                                    source: parts[0].to_string(),
                                    destination: parts[1].to_string(),
                                })
                            } else {
                                None
                            }
                        }
                        "RENAME_FILE" => {
                            let parts: Vec<&str> = params.splitn(2, ' ').collect();
                            if parts.len() == 2 {
                                Some(Tool::RenameFile {
                                    old_name: parts[0].to_string(),
                                    new_name: parts[1].to_string(),
                                })
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };
                }
            }
        }
        None
    }

    pub async fn run(
        &mut self,
        config: &crate::config::Config,
        user_prompt: String,
        app: Arc<Mutex<crate::app::App>>,
    ) -> Result<(String, Vec<String>), Box<dyn std::error::Error + Send + Sync>> {
        self.run_with_streaming(config, user_prompt, app).await
    }

    pub async fn run_with_streaming(
        &mut self,
        config: &crate::config::Config,
        user_prompt: String,
        app: Arc<Mutex<crate::app::App>>,
    ) -> Result<(String, Vec<String>), Box<dyn std::error::Error + Send + Sync>> {
        // Add system message if this is the first interaction
        if self.messages.is_empty() {
            self.messages.push(Message {
                role: "system".to_string(),
                content: Self::get_system_prompt(),
            });
        }

        // Add planning reminder for complex tasks
        if self.messages.len() == 1 {
            // Only add once, after system message
            self.messages.push(Message {
                role: "system".to_string(),
                content: "REMINDER: If the user's request is complex (3+ steps, multiple files), start with CREATE_PLAN. For simple tasks, use tools directly.".to_string(),
            });
        }

        // Add task complexity analysis
        let user_input_lower = user_prompt.to_lowercase();
        let complex_keywords = [
            "create",
            "build",
            "implement",
            "develop",
            "setup",
            "add",
            "design",
            "refactor",
            "migrate",
        ];
        let has_complex_keywords = complex_keywords
            .iter()
            .any(|&keyword| user_input_lower.contains(keyword));
        let is_complex = has_complex_keywords
            || user_input_lower.contains("app")
            || user_input_lower.contains("application")
            || user_input_lower.contains("system")
            || user_input_lower.contains("api")
            || user_input_lower.contains("server")
            || user_input_lower.contains("database");

        if is_complex && self.messages.len() == 2 {
            // Only for first user message
            self.messages.push(Message {
                role: "system".to_string(),
                content: "This appears to be a complex task. Please start with CREATE_PLAN to break it down into manageable steps.".to_string(),
            });
        }

        // Add user message
        self.messages.push(Message {
            role: "user".to_string(),
            content: user_prompt.clone(),
        });

        let mut all_tool_logs = Vec::new();
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 8;

        loop {
            attempts += 1;

            // Start streaming for this response
            {
                let mut app_guard = app.lock().await;
                app_guard.start_streaming();
            }

            // Create a string to collect the full response
            let mut full_response = String::new();

            // Get streaming response from LLM
            let mut stream = match crate::llm::stream_llm_response(&config.llm, &self.messages)
                .await
            {
                Ok(stream) => stream,
                Err(e) => {
                    let mut app_guard = app.lock().await;
                    app_guard
                        .finish_streaming("Error: Failed to start streaming response".to_string());
                    return Err(Box::new(e));
                }
            };

            // Collect tokens from the stream
            let mut token_stream = String::new();
            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        if !chunk.is_empty() {
                            token_stream.push_str(&chunk);
                            full_response.push_str(&chunk);
                            // Update streaming message with brief lock
                            {
                                let mut app_guard = app.lock().await;
                                app_guard.update_streaming_message(&chunk);
                            }
                        }
                    }
                    Err(e) => {
                        let mut app_guard = app.lock().await;
                        app_guard.finish_streaming(format!("Error in streaming: {}", e));
                        return Err(Box::new(e));
                    }
                }
            }

            {
                let mut app_guard = app.lock().await;
                app_guard.increment_requests();
            }

            // Check if response contains a tool call
            if let Some(tool) = self.parse_tool_call(&full_response) {
                let mut tool_logs = Vec::new();

                // Log the tool execution
                let tool_name = match &tool {
                    Tool::ReadFile { path } => format!("READ_FILE {}", path),
                    Tool::WriteFile { path, content: _ } => format!("WRITE_FILE {}", path),
                    Tool::AppendFile { path, content: _ } => format!("APPEND_FILE {}", path),
                    Tool::SearchReplace {
                        path,
                        old_string: _,
                        new_string: _,
                    } => format!("SEARCH_REPLACE {}", path),
                    Tool::DeleteFile { path } => format!("DELETE_FILE {}", path),
                    Tool::ListFiles { path } => format!("LIST_FILES {}", path),
                    Tool::ListFilesRecursive { path } => format!("LIST_FILES_RECURSIVE {}", path),
                    Tool::CreateDirectory { path } => format!("CREATE_DIRECTORY {}", path),
                    Tool::GrepSearch { pattern, path: _ } => format!("GREP_SEARCH {}", pattern),
                    Tool::GlobSearch { pattern } => format!("GLOB_SEARCH {}", pattern),
                    Tool::FuzzyFind { pattern, path: _ } => format!("FUZZY_FIND {}", pattern),
                    Tool::IndexCodebase { path } => format!("INDEX_CODEBASE {}", path),
                    Tool::SearchIndex { query } => format!("SEARCH_INDEX {}", query),
                    Tool::ReadUrl { url } => format!("READ_URL {}", url),
                    Tool::SearchWeb { query } => format!("SEARCH_WEB '{}'", query),
                    Tool::Remember { fact } => {
                        format!("REMEMBER ({})", fact.chars().take(50).collect::<String>())
                    }
                    Tool::Recall => "RECALL".to_string(),
                    Tool::ExecuteCode { language, code: _ } => format!("EXECUTE_CODE {}", language),
                    Tool::RunCommand { command } => format!("RUN_COMMAND {}", command),
                    Tool::GitStatus => "GIT_STATUS".to_string(),
                    Tool::GitDiff => "GIT_DIFF".to_string(),
                    Tool::GitCommit { message } => format!("GIT_COMMIT \"{}\"", message),
                    Tool::GitLog { count } => format!(
                        "GIT_LOG {}",
                        count.map_or("all".to_string(), |n| n.to_string())
                    ),
                    Tool::RunLint { language, path: _ } => format!("RUN_LINT {}", language),
                    Tool::RunTests { framework, path: _ } => format!("RUN_TESTS {}", framework),
                    Tool::InstallPackage { manager, package } => {
                        format!("INSTALL_PACKAGE {} {}", manager, package)
                    }
                    Tool::CreatePlan { task, steps } => {
                        format!("CREATE_PLAN \"{}\" ({} steps)", task, steps.len())
                    }
                    Tool::UpdatePlan { completed_step } => {
                        format!("UPDATE_PLAN step {}", completed_step)
                    }
                    Tool::ClearPlan => "CLEAR_PLAN".to_string(),
                    Tool::GetTime => "GET_TIME".to_string(),
                    Tool::GetOsInfo => "GET_OS_INFO".to_string(),
                    Tool::CopyFile {
                        source,
                        destination,
                    } => format!("COPY_FILE {} -> {}", source, destination),
                    Tool::MoveFile {
                        source,
                        destination,
                    } => format!("MOVE_FILE {} -> {}", source, destination),
                    Tool::RenameFile { old_name, new_name } => {
                        format!("RENAME_FILE {} -> {}", old_name, new_name)
                    }
                };
                tool_logs.push(format!("[ATTEMPT {}] Executing {}", attempts, tool_name));

                // Execute the tool
                let tool_result = match tool.execute(&config.web) {
                    Ok(result) => {
                        {
                            let mut app_guard = app.lock().await;
                            app_guard.increment_tools_executed();
                        }
                        tool_logs.push(format!("[SUCCESS] {}", result));
                        result
                    }
                    Err(e) => {
                        let error_msg = format!("[ERROR] {}", e);
                        tool_logs.push(error_msg.clone());
                        format!("Tool failed: {}. Please try a different approach or check if the path/command is correct.", e)
                    }
                };
                all_tool_logs.extend(tool_logs);

                // Add assistant message and tool result to conversation
                self.messages.push(Message {
                    role: "assistant".to_string(),
                    content: full_response.clone(),
                });

                self.messages.push(Message {
                    role: "user".to_string(),
                    content: format!("Tool result: {}", tool_result),
                });

                // Check if we should continue or if the task is complete
                if attempts >= MAX_ATTEMPTS {
                    // Get final response after max attempts (non-streaming for final response)
                    let (final_response, final_tokens) =
                        crate::llm::ask_llm_with_messages(&config.llm, &self.messages).await?;
                    {
                        let mut app_guard = app.lock().await;
                        app_guard.increment_tokens(final_tokens);
                        app_guard.finish_streaming(final_response.clone());
                    }

                    self.messages.push(Message {
                        role: "assistant".to_string(),
                        content: final_response.clone(),
                    });

                    all_tool_logs.push(format!(
                        "[WARNING] Reached maximum attempts ({})",
                        MAX_ATTEMPTS
                    ));
                    return Ok((final_response, all_tool_logs));
                }

                // Continue to next iteration to see if more tools are needed
                continue;
            } else {
                // No tool call, task appears to be complete
                self.messages.push(Message {
                    role: "assistant".to_string(),
                    content: full_response.clone(),
                });

                if attempts > 1 {
                    all_tool_logs.push(format!(
                        "[COMPLETE] Task completed after {} attempts",
                        attempts
                    ));
                }

                {
                    let mut app_guard = app.lock().await;
                    app_guard.finish_streaming(full_response.clone());
                }
                return Ok((full_response, all_tool_logs));
            }
        }
    }
}
