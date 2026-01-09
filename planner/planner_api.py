"""
Planner API: JSON-RPC over WebSocket API for task planning and step generation
"""
import json
import asyncio
from typing import Dict, Any, Optional
from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from pydantic import BaseModel
from task_detector import TaskDetector, TaskSource
from step_generator import StepGenerator


app = FastAPI(title="Sentinel Planner API")

task_detector = TaskDetector()
step_generator = StepGenerator()


class CreateTaskRequest(BaseModel):
    task_name: str
    task_description: Optional[str] = None
    task_source: Optional[str] = None
    context: Optional[Dict[str, Any]] = None
    scheduling: Optional[Dict[str, Any]] = None
    automation: Optional[Dict[str, Any]] = None


class ChatCommandRequest(BaseModel):
    command: str
    context: Optional[Dict[str, Any]] = None


class GenerateWorkflowRequest(BaseModel):
    task_name: str
    task_description: Optional[str] = None
    context: Optional[Dict[str, Any]] = None


@app.post("/api/v1/detect-task")
async def detect_task_from_chat(request: ChatCommandRequest) -> Dict[str, Any]:
    """
    Detect and parse task from natural language chat command
    """
    try:
        detected = task_detector.detect_from_chat(request.command)
        
        # Generate workflow steps
        workflow = step_generator.generate_steps(
            detected["task_name"],
            request.context,
        )
        
        # Validate workflow
        is_valid, error = step_generator.validate_workflow(workflow)
        if not is_valid:
            return {
                "success": False,
                "error": error,
            }
        
        return {
            "success": True,
            "task": {
                "task_name": detected["task_name"],
                "task_source": detected["task_source"],
                "scheduling": detected.get("scheduling"),
                "automation": {
                    "is_repetitive": detected.get("is_repetitive", False),
                    "auto_run_enabled": detected.get("auto_approved", False),
                },
                "workflow": workflow,
            },
        }
    except Exception as e:
        return {
            "success": False,
            "error": str(e),
        }


@app.post("/api/v1/generate-workflow")
async def generate_workflow(request: GenerateWorkflowRequest) -> Dict[str, Any]:
    """
    Generate workflow steps from task information
    """
    try:
        workflow = step_generator.generate_steps(
            request.task_name,
            request.task_description,
            request.context,
        )
        
        # Validate workflow
        is_valid, error = step_generator.validate_workflow(workflow)
        if not is_valid:
            return {
                "success": False,
                "error": error,
            }
        
        return {
            "success": True,
            "workflow": workflow,
        }
    except Exception as e:
        return {
            "success": False,
            "error": str(e),
        }


@app.post("/api/v1/suggest-task")
async def suggest_task(
    behavior_pattern: str,
    frequency: int,
    last_executed: Optional[str] = None,
) -> Dict[str, Any]:
    """
    Suggest a task based on detected behavior patterns
    """
    try:
        from datetime import datetime
        last_exec = None
        if last_executed:
            last_exec = datetime.fromisoformat(last_executed)
        
        suggestion = task_detector.suggest_task_from_behavior(
            behavior_pattern,
            frequency,
            last_exec,
        )
        
        if suggestion:
            # Generate workflow for suggested task
            workflow = step_generator.generate_steps(
                suggestion["task_name"],
                None,
            )
            
            return {
                "success": True,
                "suggestion": {
                    **suggestion,
                    "workflow": workflow,
                },
            }
        else:
            return {
                "success": False,
                "message": "No suggestion at this time",
            }
    except Exception as e:
        return {
            "success": False,
            "error": str(e),
        }


@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    """
    WebSocket endpoint for JSON-RPC communication
    """
    await websocket.accept()
    
    try:
        while True:
            data = await websocket.receive_text()
            message = json.loads(data)
            
            # JSON-RPC format
            method = message.get("method")
            params = message.get("params", {})
            request_id = message.get("id")
            
            response = await handle_jsonrpc_request(method, params)
            
            await websocket.send_text(json.dumps({
                "jsonrpc": "2.0",
                "id": request_id,
                "result": response,
            }))
    except WebSocketDisconnect:
        print("WebSocket disconnected")
    except Exception as e:
        await websocket.send_text(json.dumps({
            "jsonrpc": "2.0",
            "error": {
                "code": -32000,
                "message": str(e),
            },
        }))


async def handle_jsonrpc_request(method: str, params: Dict[str, Any]) -> Dict[str, Any]:
    """Handle JSON-RPC method calls"""
    
    if method == "detect_task":
        command = params.get("command", "")
        context = params.get("context")
        detected = task_detector.detect_from_chat(command)
        workflow = step_generator.generate_steps(
            detected["task_name"],
            context,
        )
        return {
            "task_name": detected["task_name"],
            "task_source": detected["task_source"],
            "scheduling": detected.get("scheduling"),
            "automation": detected.get("automation", {}),
            "workflow": workflow,
        }
    
    elif method == "generate_workflow":
        task_name = params.get("task_name", "")
        task_description = params.get("task_description")
        context = params.get("context")
        workflow = step_generator.generate_steps(
            task_name,
            task_description,
            context,
        )
        return {"workflow": workflow}
    
    elif method == "suggest_task":
        behavior_pattern = params.get("behavior_pattern", "")
        frequency = params.get("frequency", 0)
        last_executed = params.get("last_executed")
        suggestion = task_detector.suggest_task_from_behavior(
            behavior_pattern,
            frequency,
            last_executed,
        )
        return {"suggestion": suggestion}
    
    else:
        raise ValueError(f"Unknown method: {method}")


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)

