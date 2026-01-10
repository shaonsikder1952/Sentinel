import os
from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
import uvicorn
from groq import Groq
from dotenv import load_dotenv

# Load environment variables
load_dotenv()

app = FastAPI(title="Sentinel Planner API")

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Initialize Groq client
groq_client = Groq(api_key=os.getenv("GROQ_API_KEY"))

class ChatMessage(BaseModel):
    message: str

class TaskRequest(BaseModel):
    description: str
    priority: str = "medium"

@app.get("/")
async def root():
    return {"message": "Sentinel Planner API", "status": "running"}

@app.post("/chat")
async def chat(msg: ChatMessage):
    try:
        completion = groq_client.chat.completions.create(
            model="llama-3.3-70b-versatile",
            messages=[{"role": "user", "content": msg.message}],
            temperature=0.7,
            max_tokens=1024,
        )
        return {"response": completion.choices[0].message.content}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/task/create")
async def create_task(task: TaskRequest):
    try:
        # Use AI to analyze and structure the task
        completion = groq_client.chat.completions.create(
            model="llama-3.3-70b-versatile",
            messages=[{
                "role": "system",
                "content": "You are a task planning assistant. Break down tasks into actionable steps."
            }, {
                "role": "user",
                "content": f"Break down this task: {task.description}"
            }],
            temperature=0.5,
            max_tokens=512,
        )
        return {
            "task_id": "task_" + str(hash(task.description)),
            "description": task.description,
            "priority": task.priority,
            "steps": completion.choices[0].message.content,
            "status": "pending"
        }
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

@app.get("/health")
async def health():
    return {"status": "healthy", "ai_available": bool(os.getenv("GROQ_API_KEY"))}

if __name__ == "__main__":
    port = int(os.getenv("API_PORT", 8000))
    uvicorn.run(app, host="0.0.0.0", port=port)
