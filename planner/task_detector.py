"""
Task Detector: Detects tasks from natural language commands and user behavior
"""
import re
from typing import Optional, Dict, List, Tuple
from datetime import datetime, timedelta
from enum import Enum
import json


class TaskSource(str, Enum):
    USER_MANUAL = "user_manual"
    USER_CHAT = "user_chat"
    AI_AUTO_DETECTED = "ai_auto_detected"
    AI_SUGGESTED = "ai_suggested"
    SCHEDULED = "scheduled"


class TaskDetector:
    """Detects and parses tasks from natural language commands"""
    
    def __init__(self):
        self.patterns = {
            "schedule": [
                r"(?:schedule|run|do|execute)\s+(?:this|it|task)?\s*(?:every|on|at)?\s*(daily|weekly|monthly|monday|tuesday|wednesday|thursday|friday|saturday|sunday)",
                r"(?:every|each)\s+(\d+)\s*(?:day|week|month|hour|minute)s?",
                r"at\s+(\d{1,2}):(\d{2})\s*(?:am|pm)?",
            ],
            "repetitive": [
                r"(?:weekly|daily|monthly|recurring|repeat|automate)",
                r"every\s+(?:monday|tuesday|wednesday|thursday|friday|saturday|sunday)",
            ],
            "approval": [
                r"(?:auto|automatic|without approval|skip approval)",
                r"(?:require|need|ask for)\s+approval",
            ],
        }
    
    def detect_from_chat(self, command: str) -> Dict:
        """
        Parse a natural language command and extract task information
        
        Args:
            command: Natural language command (e.g., "Do weekly KPI report")
            
        Returns:
            Dictionary with parsed task information
        """
        command_lower = command.lower().strip()
        
        # Extract task name (remove scheduling/automation keywords)
        task_name = self._extract_task_name(command)
        
        # Detect scheduling
        scheduling = self._detect_scheduling(command_lower)
        
        # Detect if repetitive/automated
        is_repetitive = self._is_repetitive(command_lower)
        auto_approved = self._should_auto_approve(command_lower)
        
        return {
            "task_name": task_name,
            "task_source": TaskSource.USER_CHAT.value,
            "scheduling": scheduling,
            "is_repetitive": is_repetitive,
            "auto_approved": auto_approved,
            "command": command,
        }
    
    def _extract_task_name(self, command: str) -> str:
        """Extract the core task name from command"""
        # Remove common scheduling/automation phrases
        patterns_to_remove = [
            r"schedule\s+",
            r"run\s+",
            r"do\s+",
            r"execute\s+",
            r"every\s+\w+",
            r"at\s+\d{1,2}:\d{2}",
            r"weekly\s+",
            r"daily\s+",
            r"monthly\s+",
            r"automatically\s+",
            r"auto\s+",
        ]
        
        task_name = command
        for pattern in patterns_to_remove:
            task_name = re.sub(pattern, "", task_name, flags=re.IGNORECASE)
        
        return task_name.strip()
    
    def _detect_scheduling(self, command: str) -> Optional[Dict]:
        """Detect scheduling information from command"""
        scheduling = None
        
        # Check for time
        time_match = re.search(r"at\s+(\d{1,2}):(\d{2})\s*(am|pm)?", command, re.IGNORECASE)
        time_str = None
        if time_match:
            hour = int(time_match.group(1))
            minute = int(time_match.group(2))
            am_pm = time_match.group(3)
            if am_pm and am_pm.lower() == "pm" and hour != 12:
                hour += 12
            elif am_pm and am_pm.lower() == "am" and hour == 12:
                hour = 0
            time_str = f"{hour:02d}:{minute:02d}"
        
        # Check for frequency
        if re.search(r"daily|every day", command, re.IGNORECASE):
            scheduling = {
                "schedule_type": "recurring",
                "frequency": "daily",
                "time": time_str or "09:00",
            }
        elif re.search(r"weekly|every week", command, re.IGNORECASE):
            # Try to detect day of week
            days_map = {
                "monday": 0, "tuesday": 1, "wednesday": 2, "thursday": 3,
                "friday": 4, "saturday": 5, "sunday": 6,
            }
            day_of_week = None
            for day_name, day_num in days_map.items():
                if day_name in command.lower():
                    day_of_week = day_num
                    break
            
            scheduling = {
                "schedule_type": "recurring",
                "frequency": "weekly",
                "days_of_week": [day_of_week] if day_of_week is not None else [0],  # Default Monday
                "time": time_str or "09:00",
            }
        elif re.search(r"monthly|every month", command, re.IGNORECASE):
            scheduling = {
                "schedule_type": "recurring",
                "frequency": "monthly",
                "time": time_str or "09:00",
            }
        elif re.search(r"once|one time|single", command, re.IGNORECASE):
            # One-time scheduled task
            scheduling = {
                "schedule_type": "once",
                "time": time_str or "09:00",
            }
        
        if scheduling:
            # Calculate next_run (simplified - would need proper date calculation)
            from dateutil.relativedelta import relativedelta
            now = datetime.now()
            if scheduling["schedule_type"] == "once":
                next_run = now.replace(hour=int(time_str.split(":")[0]), minute=int(time_str.split(":")[1]))
                if next_run < now:
                    next_run += timedelta(days=1)
            else:
                next_run = now + timedelta(days=1)
            
            scheduling["next_run"] = next_run.isoformat()
            scheduling["enabled"] = True
        
        return scheduling
    
    def _is_repetitive(self, command: str) -> bool:
        """Check if task should be marked as repetitive"""
        repetitive_keywords = [
            "weekly", "daily", "monthly", "recurring", "repeat",
            "every", "automate", "automatic",
        ]
        return any(keyword in command for keyword in repetitive_keywords)
    
    def _should_auto_approve(self, command: str) -> bool:
        """Check if task should be auto-approved"""
        auto_keywords = ["auto", "automatic", "without approval", "skip approval"]
        return any(keyword in command for keyword in auto_keywords)
    
    def suggest_task_from_behavior(
        self,
        behavior_pattern: str,
        frequency: int,
        last_executed: Optional[datetime] = None,
    ) -> Optional[Dict]:
        """
        Suggest a task based on detected user behavior patterns
        
        Args:
            behavior_pattern: Description of the behavior (e.g., "login to dashboard")
            frequency: How many times this behavior was observed
            last_executed: When this was last executed
            
        Returns:
            Suggested task information or None
        """
        # Only suggest if behavior is frequent enough
        if frequency < 3:
            return None
        
        # Check if enough time has passed since last execution
        if last_executed:
            days_since = (datetime.now() - last_executed).days
            if days_since < 7:  # Don't suggest if done recently
                return None
        
        return {
            "task_name": f"Automated {behavior_pattern}",
            "task_source": TaskSource.AI_SUGGESTED.value,
            "is_repetitive": True,
            "auto_approved": False,  # Always require approval for suggested tasks
            "behavior_pattern": behavior_pattern,
            "frequency": frequency,
        }


if __name__ == "__main__":
    detector = TaskDetector()
    
    # Test cases
    test_commands = [
        "Do weekly KPI report",
        "Schedule monthly revenue report every Monday at 9:00 AM",
        "Run daily backup automatically",
        "Execute task every Tuesday at 2:00 PM",
    ]
    
    for cmd in test_commands:
        result = detector.detect_from_chat(cmd)
        print(f"Command: {cmd}")
        print(f"Result: {json.dumps(result, indent=2, default=str)}")
        print()

