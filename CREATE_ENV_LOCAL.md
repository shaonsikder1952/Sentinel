# Creating .env File on Your Local Machine

## Location
The `.env` file goes in the **ROOT** of your Sentinel folder:

```
Sentinel/              ← CREATE .env HERE
├── .env              ← This file (you create it)
├── .env.example      ← Template (already exists)
├── planner/
├── overlay_ui/
└── README.md
```

---

## Method 1: Copy from Template (Easiest)

### On macOS/Linux:
```bash
# Open terminal in the Sentinel folder
cd /path/to/Sentinel

# Copy the example file
cp .env.example .env

# Edit it
nano .env
```

### On Windows (Command Prompt):
```cmd
# Navigate to Sentinel folder
cd C:\\path\\to\\Sentinel

# Copy the example file
copy .env.example .env

# Edit it
notepad .env
```

### On Windows (PowerShell):
```powershell
# Navigate to Sentinel folder
cd C:\\path\\to\\Sentinel

# Copy the example file
Copy-Item .env.example .env

# Edit it
notepad .env
```

---

## Method 2: Create from Scratch

### Using Text Editor (Any OS):

1. **Open your Sentinel folder** in File Explorer/Finder
2. **Create a new file** called `.env` (note the dot at the start!)
3. **Open it** with any text editor (Notepad, TextEdit, VS Code, etc.)
4. **Add this content**:

```
# Groq API Configuration
GROQ_API_KEY=your_groq_api_key_here

# API Server Configuration
API_HOST=127.0.0.1
API_PORT=8000
```

5. **Replace** `your_groq_api_key_here` with your actual API key
6. **Save** the file

---

## Method 3: Create via Command Line

### On macOS/Linux:
```bash
cd /path/to/Sentinel

cat > .env << 'ENDFILE'
# Groq API Configuration
GROQ_API_KEY=your_groq_api_key_here

# API Server Configuration
API_HOST=127.0.0.1
API_PORT=8000
ENDFILE

# Now edit it to add your real API key
nano .env
```

### On Windows (PowerShell):
```powershell
cd C:\\path\\to\\Sentinel

@"
# Groq API Configuration
GROQ_API_KEY=your_groq_api_key_here

# API Server Configuration
API_HOST=127.0.0.1
API_PORT=8000
"@ | Out-File -FilePath .env -Encoding UTF8

# Now edit it
notepad .env
```

---

## Getting Your Groq API Key

1. **Visit**: https://console.groq.com
2. **Sign up** (free, no credit card required)
3. **Click** "API Keys" in the left sidebar
4. **Create** a new API key
5. **Copy** the key (starts with `gsk_`)
6. **Paste** it into your `.env` file

---

## Final .env File Should Look Like:

```env
# Groq API Configuration
GROQ_API_KEY=gsk_abc123xyz789youractualkey456def

# API Server Configuration
API_HOST=127.0.0.1
API_PORT=8000
```

---

## Verify It Works

### Test that Python can read it:
```bash
cd planner
python3 -c "import os; from dotenv import load_dotenv; load_dotenv('../.env'); print('API Key loaded:', os.getenv('GROQ_API_KEY')[:20] + '...')"
```

You should see:
```
API Key loaded: gsk_abc123xyz789you...
```

---

## Common Issues

### Issue: "File .env not found"
**Solution**: Make sure you're in the Sentinel root folder, not in planner/ or overlay_ui/

### Issue: "Invalid API key"
**Solution**: 
- Check there are no extra spaces
- Make sure you copied the entire key
- Verify key at https://console.groq.com

### Issue: "Can't see .env file"
**Solution**: Files starting with `.` are hidden by default

**On macOS Finder:**
- Press `Cmd + Shift + .` to show hidden files

**On Windows Explorer:**
- View tab → Show → Hidden items (check it)

**On Linux:**
- Press `Ctrl + H` in file manager

### Issue: "Module 'dotenv' not found"
**Solution**:
```bash
cd planner
pip install python-dotenv
```

---

## Security Reminder

⚠️ **NEVER commit .env to Git!**

The `.env` file contains your secret API key. It's already in `.gitignore`, but double-check:

```bash
cat .gitignore | grep .env
```

You should see `.env` listed.

