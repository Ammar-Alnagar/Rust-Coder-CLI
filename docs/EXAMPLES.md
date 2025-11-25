# Usage Examples

This document provides practical examples of using Rust TUI Coder for various development tasks.

## Table of Contents

1. [Basic File Operations](#basic-file-operations)
2. [Code Generation](#code-generation)
3. [Refactoring Code](#refactoring-code)
4. [Debugging Assistance](#debugging-assistance)
5. [Project Setup](#project-setup)
6. [Testing](#testing)
7. [Documentation](#documentation)
8. [Git Workflows](#git-workflows)
9. [Data Processing](#data-processing)
10. [Advanced Workflows](#advanced-workflows)

---

## Basic File Operations

### Example 1: Create a Simple File

**User:**
```
Create a README.md file with a title "My Project" and a brief description
```

**What happens:**
- AI writes content
- Saves to README.md
- Confirms creation

**Result:**
```markdown
# My Project

A brief description of what this project does.
```

---

### Example 2: Read and Analyze a File

**User:**
```
Read the file src/main.rs and tell me what it does
```

**What happens:**
- AI reads the file using `read_file` tool
- Analyzes the code
- Explains functionality

---

### Example 3: Modify Existing Content

**User:**
```
In config.toml, change the model_name from gpt-3.5-turbo to gpt-4
```

**What happens:**
- AI reads config.toml
- Uses `search_and_replace` tool
- Updates the model name
- Confirms the change

---

## Code Generation

### Example 4: Generate a Python Script

**User:**
```
Create a Python script that reads a CSV file and calculates the average of the 'price' column
```

**What happens:**
```python
# average_price.py
import csv

def calculate_average_price(filename):
    total = 0
    count = 0
    
    with open(filename, 'r') as file:
        reader = csv.DictReader(file)
        for row in reader:
            total += float(row['price'])
            count += 1
    
    return total / count if count > 0 else 0

if __name__ == "__main__":
    avg = calculate_average_price('data.csv')
    print(f"Average price: ${avg:.2f}")
```

- AI generates the code
- Saves to `average_price.py`
- Explains usage

---

### Example 5: Generate a Web API Endpoint

**User:**
```
Create a Node.js Express endpoint that handles POST requests to /users and saves the data to a JSON file
```

**What happens:**
```javascript
// server.js
const express = require('express');
const fs = require('fs');
const app = express();

app.use(express.json());

app.post('/users', (req, res) => {
    const userData = req.body;
    const users = JSON.parse(fs.readFileSync('users.json', 'utf8') || '[]');
    users.push(userData);
    fs.writeFileSync('users.json', JSON.stringify(users, null, 2));
    res.status(201).json({ message: 'User created', user: userData });
});

app.listen(3000, () => console.log('Server running on port 3000'));
```

---

### Example 6: Generate Test Cases

**User:**
```
Create unit tests for the calculate_average_price function using pytest
```

**What happens:**
- AI creates `test_average_price.py`
- Includes multiple test cases
- Tests edge cases (empty file, invalid data, etc.)

---

## Refactoring Code

### Example 7: Extract Function

**User:**
```
Read server.js and extract the user validation logic into a separate function
```

**What happens:**
- AI reads the file
- Identifies validation code
- Creates new function
- Updates calls to use new function
- Shows the refactored code

---

### Example 8: Add Error Handling

**User:**
```
Add try-catch error handling to all file operations in backup.py
```

**What happens:**
- AI reads the file
- Identifies file operations
- Adds appropriate error handling
- Updates the file

---

### Example 9: Convert Callback to Async/Await

**User:**
```
Convert the callback-based code in api.js to use async/await
```

**What happens:**
- AI reads the file
- Converts callbacks to promises
- Uses async/await syntax
- Updates the file

---

## Debugging Assistance

### Example 10: Find Bug

**User:**
```
This function is giving wrong results. Can you help?

def calculate_discount(price, percent):
    discount = price * percent
    return price - discount
```

**AI Response:**
```
The bug is that you're using the percentage as a decimal multiplier without dividing by 100.

Fixed version:
def calculate_discount(price, percent):
    discount = price * (percent / 100)
    return price - discount

Example: calculate_discount(100, 20) now correctly returns 80
```

---

### Example 11: Debug with Logs

**User:**
```
Add debug logging to process_data.py to help track down where it's failing
```

**What happens:**
- AI adds logging statements
- Includes relevant variable values
- Updates the file

---

### Example 12: Analyze Error Messages

**User:**
```
I'm getting this error: "TypeError: Cannot read property 'name' of undefined"
Here's my code: [paste code]
```

**AI Response:**
- Identifies the issue
- Explains why it happens
- Suggests fixes
- Can update the code if requested

---

## Project Setup

### Example 13: Initialize a Python Project

**User:**
```
Set up a new Python project structure with virtual environment, requirements.txt, and basic folders
```

**What happens:**
- Creates directory structure
- Generates requirements.txt
- Creates __init__.py files
- Provides setup instructions

---

### Example 14: Create a React App Structure

**User:**
```
Create a basic React app structure with components, hooks, and utils folders
```

**What happens:**
```
my-app/
├── src/
│   ├── components/
│   │   └── App.jsx
│   ├── hooks/
│   │   └── useCustomHook.js
│   ├── utils/
│   │   └── helpers.js
│   └── index.js
├── public/
│   └── index.html
└── package.json
```

---

### Example 15: Set Up Configuration Files

**User:**
```
Create a .eslintrc.json and .prettierrc for my JavaScript project
```

**What happens:**
- Creates ESLint configuration
- Creates Prettier configuration
- Both with sensible defaults
- Explains how to customize

---

## Testing

### Example 16: Generate Test Data

**User:**
```
Create a JSON file with 10 sample user records including name, email, and age
```

**What happens:**
```json
[
  {
    "id": 1,
    "name": "Alice Johnson",
    "email": "alice@example.com",
    "age": 28
  },
  // ... 9 more records
]
```

---

### Example 17: Create Integration Test

**User:**
```
Write an integration test that tests the entire user registration flow
```

**What happens:**
- AI creates test file
- Includes setup and teardown
- Tests complete workflow
- Adds assertions

---

### Example 18: Run Tests and Analyze

**User:**
```
Run the tests in test_api.py and show me the results
```

**What happens:**
- AI executes `pytest test_api.py`
- Shows output
- Explains failures if any
- Suggests fixes

---

## Documentation

### Example 19: Generate Function Documentation

**User:**
```
Add docstrings to all functions in utils.py following Google style
```

**What happens:**
- AI reads the file
- Adds comprehensive docstrings
- Includes parameters, returns, examples
- Updates the file

---

### Example 20: Create API Documentation

**User:**
```
Document all the API endpoints in server.js in Markdown format
```

**What happens:**
- AI analyzes endpoints
- Creates API.md
- Includes request/response examples
- Lists all routes

---

### Example 21: Generate README

**User:**
```
Create a comprehensive README for this project based on the files in the current directory
```

**What happens:**
- AI scans project structure
- Generates README with:
  - Project description
  - Installation instructions
  - Usage examples
  - API documentation
  - Contributing guidelines

---

## Git Workflows

### Example 22: Check Status Before Commit

**User:**
```
Show me the current git status
```

**What happens:**
- AI runs `git_status` tool
- Shows modified, staged, untracked files
- Can suggest what to commit

---

### Example 23: Generate Commit Message

**User:**
```
Based on the changes I've made, suggest a good commit message
```

**What happens:**
- AI reviews git status
- Suggests descriptive commit message
- Follows conventional commit format

---

## Data Processing

### Example 24: Process CSV File

**User:**
```
Read sales.csv, filter rows where amount > 1000, and save to high_value_sales.csv
```

**What happens:**
- AI writes Python script
- Processes the data
- Saves filtered results
- Shows summary statistics

---

### Example 25: Convert JSON to CSV

**User:**
```
Convert users.json to users.csv with columns: id, name, email
```

**What happens:**
- AI reads JSON
- Converts to CSV format
- Saves to file
- Confirms conversion

---

### Example 26: Data Analysis

**User:**
```
Analyze data.csv and show me:
- Total number of records
- Average of the 'value' column
- Top 5 highest values
```

**What happens:**
- AI processes the data
- Calculates statistics
- Presents results clearly

---

## Advanced Workflows

### Example 27: Multi-Step Project Creation

**User:**
```
Create a complete REST API with user authentication
```

**AI Creates Plan:**
```markdown
## Phase 1: Setup
- [ ] Create project structure
- [ ] Initialize package.json
- [ ] Install dependencies

## Phase 2: Database
- [ ] Create user schema
- [ ] Set up database connection
- [ ] Create migration files

## Phase 3: Authentication
- [ ] Implement JWT auth
- [ ] Create login endpoint
- [ ] Create register endpoint

## Phase 4: User Routes
- [ ] Get user profile
- [ ] Update user
- [ ] Delete user

## Phase 5: Testing
- [ ] Write unit tests
- [ ] Write integration tests
- [ ] Test authentication flow
```

**Then:**
```
Implement Phase 1 of the plan
```

---

### Example 28: Code Review

**User:**
```
Review the code in app.py and suggest improvements for:
- Performance
- Readability
- Best practices
```

**What happens:**
- AI analyzes code
- Points out issues
- Suggests improvements
- Can apply changes if requested

---

### Example 29: Migration Between Languages

**User:**
```
Convert the function in utils.js to Python
```

**What happens:**
- AI reads JavaScript code
- Converts to Python equivalent
- Maintains functionality
- Explains differences

---

### Example 30: Batch File Operations

**User:**
```
Read all .txt files in the docs/ folder and create a combined.txt with all their contents
```

**What happens:**
- AI lists directory
- Reads each file
- Combines content
- Saves to combined.txt

---

## Real-World Scenario Examples

### Scenario A: Bug Fix Workflow

1. **User:** "I have a bug in payment.py where discounts aren't being applied correctly"
2. **AI:** Reads the file, finds the issue
3. **User:** "Can you fix it?"
4. **AI:** Fixes the bug
5. **User:** "Add a test to prevent this in the future"
6. **AI:** Creates test case

---

### Scenario B: Feature Addition

1. **User:** "Add a rate limiting feature to the API"
2. **AI:** Creates plan with steps
3. **User:** "Implement step 1"
4. **AI:** Creates middleware file
5. **User:** "Continue with step 2"
6. **AI:** Integrates middleware
7. **User:** "Add tests"
8. **AI:** Creates test cases

---

### Scenario C: Code Exploration

1. **User:** "Show me the structure of this project"
2. **AI:** Lists directory recursively
3. **User:** "What does main.rs do?"
4. **AI:** Reads and explains
5. **User:** "How does it connect to the database?"
6. **AI:** Finds and explains database code

---

## Tips for Best Results

### Be Specific
❌ "Make it better"
✅ "Add input validation and error messages to the login form"

### Provide Context
❌ "Fix the bug"
✅ "The calculate_total function returns NaN when the cart is empty. Fix this."

### Work Iteratively
Instead of asking for everything at once, work step-by-step and refine as you go.

### Use the Plan Feature
For complex tasks, ask for a plan first, then implement step by step.

### Review Tool Logs
Check the tool logs to see exactly what operations were performed.

---

## Command Reference Quick Guide

| Task | Example Command |
|------|-----------------|
| Create file | "Create a file named test.py" |
| Read file | "Show me the contents of config.json" |
| Modify file | "Update the API key in settings.py" |
| Delete file | "Remove the old_backup.sql file" |
| List files | "Show me all files in the src/ directory" |
| Run code | "Execute the script.py file" |
| Create plan | "Create a plan to build a blog system" |
| Check git | "Show git status" |
| Get stats | "/stats" |
| Quit | "/quit" or Ctrl+C |

---

For more examples and inspiration, experiment with your own use cases! The AI is designed to help with a wide variety of development tasks.
