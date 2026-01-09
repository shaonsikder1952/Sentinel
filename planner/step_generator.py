"""
Step Generator: Generates atomic workflow steps from high-level tasks
"""
from typing import List, Dict, Optional, Any
import json
from datetime import datetime


class StepGenerator:
    """Generates workflow steps from task descriptions"""
    
    def __init__(self):
        self.action_templates = {
            "extract_table": {
                "action": "extract",
                "verification": ["schema", "sanity_check"],
            },
            "navigate_to": {
                "action": "navigate",
            },
            "click_button": {
                "action": "click",
            },
            "fill_form": {
                "action": "type",
            },
            "submit_form": {
                "action": "submit",
            },
        }
    
    def generate_steps(
        self,
        task_name: str,
        task_description: Optional[str] = None,
        context: Optional[Dict[str, Any]] = None,
    ) -> Dict:
        """
        Generate workflow steps from task information
        
        Args:
            task_name: Name of the task
            task_description: Optional detailed description
            context: Optional context (URLs, selectors, etc.)
            
        Returns:
            Workflow dictionary with steps
        """
        workflow_id = f"workflow_{datetime.now().timestamp()}"
        steps = []
        
        # Parse task to determine steps
        task_lower = task_name.lower()
        
        # Example: "Weekly KPI Report"
        if "report" in task_lower or "kpi" in task_lower:
            steps = self._generate_report_steps(task_name, context)
        elif "login" in task_lower or "authenticate" in task_lower:
            steps = self._generate_login_steps(task_name, context)
        elif "extract" in task_lower or "scrape" in task_lower:
            steps = self._generate_extraction_steps(task_name, context)
        else:
            # Generic workflow
            steps = self._generate_generic_steps(task_name, context)
        
        return {
            "workflow_id": workflow_id,
            "version": "1.0.0",
            "steps": steps,
            "metadata": {
                "created_at": datetime.utcnow().isoformat(),
                "created_by": "ai",
                "tags": self._extract_tags(task_name),
            },
        }
    
    def _generate_report_steps(
        self,
        task_name: str,
        context: Optional[Dict[str, Any]] = None,
    ) -> List[Dict]:
        """Generate steps for report generation tasks"""
        steps = []
        
        # Step 1: Navigate to dashboard/report page
        url = context.get("url") if context else None
        if url:
            steps.append({
                "step_id": "s1",
                "action": "navigate",
                "target": url,
                "parameters": {"url": url},
                "expected_schema": None,
                "verification": ["element_presence"],
                "retry_config": {"max_retries": 2, "retry_delay_ms": 1000},
                "requires_approval": False,
            })
        
        # Step 2: Wait for page load
        steps.append({
            "step_id": "s2",
            "action": "wait",
            "target": "body",
            "parameters": {"duration_ms": 2000},
            "expected_schema": None,
            "verification": [],
            "retry_config": {"max_retries": 0, "retry_delay_ms": 0},
            "requires_approval": False,
        })
        
        # Step 3: Extract table/chart data
        table_selector = context.get("table_selector", "#kpi_table") if context else "#kpi_table"
        steps.append({
            "step_id": "s3",
            "action": "extract",
            "target": table_selector,
            "parameters": None,
            "expected_schema": {
                "type": "object",
                "properties": {
                    "columns": {"type": "array"},
                    "rows": {"type": "array"},
                },
            },
            "verification": ["schema", "sanity_check"],
            "retry_config": {"max_retries": 2, "retry_delay_ms": 1000},
            "requires_approval": False,
        })
        
        # Step 4: Verify extraction
        steps.append({
            "step_id": "s4",
            "action": "verify",
            "target": table_selector,
            "parameters": None,
            "expected_schema": {
                "type": "object",
                "properties": {
                    "columns": {"type": "array"},
                    "rows": {"type": "array"},
                },
            },
            "verification": ["schema", "sanity_check"],
            "retry_config": {"max_retries": 1, "retry_delay_ms": 500},
            "requires_approval": False,
        })
        
        return steps
    
    def _generate_login_steps(
        self,
        task_name: str,
        context: Optional[Dict[str, Any]] = None,
    ) -> List[Dict]:
        """Generate steps for login/authentication tasks"""
        steps = []
        
        url = context.get("url") if context else None
        if url:
            steps.append({
                "step_id": "s1",
                "action": "navigate",
                "target": url,
                "parameters": {"url": url},
                "expected_schema": None,
                "verification": ["element_presence"],
                "retry_config": {"max_retries": 2, "retry_delay_ms": 1000},
                "requires_approval": False,
            })
        
        username_selector = context.get("username_selector", "#username") if context else "#username"
        password_selector = context.get("password_selector", "#password") if context else "#password"
        submit_selector = context.get("submit_selector", "button[type='submit']") if context else "button[type='submit']"
        
        steps.append({
            "step_id": "s2",
            "action": "type",
            "target": username_selector,
            "parameters": {"text": context.get("username", "") if context else ""},
            "expected_schema": None,
            "verification": ["element_presence"],
            "retry_config": {"max_retries": 2, "retry_delay_ms": 1000},
            "requires_approval": True,  # Login requires approval
        })
        
        steps.append({
            "step_id": "s3",
            "action": "type",
            "target": password_selector,
            "parameters": {"text": context.get("password", "") if context else ""},
            "expected_schema": None,
            "verification": ["element_presence"],
            "retry_config": {"max_retries": 2, "retry_delay_ms": 1000},
            "requires_approval": True,  # Login requires approval
        })
        
        steps.append({
            "step_id": "s4",
            "action": "submit",
            "target": submit_selector,
            "parameters": None,
            "expected_schema": None,
            "verification": ["element_presence"],
            "retry_config": {"max_retries": 2, "retry_delay_ms": 1000},
            "requires_approval": True,  # Login requires approval
        })
        
        return steps
    
    def _generate_extraction_steps(
        self,
        task_name: str,
        context: Optional[Dict[str, Any]] = None,
    ) -> List[Dict]:
        """Generate steps for data extraction tasks"""
        steps = []
        
        url = context.get("url") if context else None
        if url:
            steps.append({
                "step_id": "s1",
                "action": "navigate",
                "target": url,
                "parameters": {"url": url},
                "expected_schema": None,
                "verification": ["element_presence"],
                "retry_config": {"max_retries": 2, "retry_delay_ms": 1000},
                "requires_approval": False,
            })
        
        extract_selector = context.get("extract_selector", "body") if context else "body"
        steps.append({
            "step_id": "s2",
            "action": "extract",
            "target": extract_selector,
            "parameters": None,
            "expected_schema": context.get("expected_schema") if context else None,
            "verification": ["schema", "sanity_check"],
            "retry_config": {"max_retries": 2, "retry_delay_ms": 1000},
            "requires_approval": False,
        })
        
        return steps
    
    def _generate_generic_steps(
        self,
        task_name: str,
        context: Optional[Dict[str, Any]] = None,
    ) -> List[Dict]:
        """Generate generic workflow steps"""
        steps = []
        
        url = context.get("url") if context else None
        if url:
            steps.append({
                "step_id": "s1",
                "action": "navigate",
                "target": url,
                "parameters": {"url": url},
                "expected_schema": None,
                "verification": ["element_presence"],
                "retry_config": {"max_retries": 2, "retry_delay_ms": 1000},
                "requires_approval": False,
            })
        
        return steps
    
    def _extract_tags(self, task_name: str) -> List[str]:
        """Extract tags from task name"""
        tags = []
        task_lower = task_name.lower()
        
        if "report" in task_lower:
            tags.append("report")
        if "kpi" in task_lower:
            tags.append("kpi")
        if "weekly" in task_lower or "daily" in task_lower or "monthly" in task_lower:
            tags.append("scheduled")
        if "login" in task_lower or "auth" in task_lower:
            tags.append("authentication")
        
        return tags
    
    def validate_workflow(self, workflow: Dict) -> tuple[bool, Optional[str]]:
        """
        Validate workflow against contracts
        
        Returns:
            (is_valid, error_message)
        """
        if "steps" not in workflow:
            return False, "Workflow must contain 'steps'"
        
        if not isinstance(workflow["steps"], list):
            return False, "Workflow steps must be a list"
        
        if len(workflow["steps"]) == 0:
            return False, "Workflow must have at least one step"
        
        for i, step in enumerate(workflow["steps"]):
            if "step_id" not in step:
                return False, f"Step {i} missing 'step_id'"
            if "action" not in step:
                return False, f"Step {i} missing 'action'"
            if "target" not in step:
                return False, f"Step {i} missing 'target'"
        
        return True, None


if __name__ == "__main__":
    generator = StepGenerator()
    
    # Test workflow generation
    workflow = generator.generate_steps(
        "Weekly KPI Report",
        context={"url": "https://dashboard.example.com/kpi", "table_selector": "#kpi_table"},
    )
    
    print(json.dumps(workflow, indent=2))
    
    # Validate
    is_valid, error = generator.validate_workflow(workflow)
    print(f"\nValidation: {'PASSED' if is_valid else f'FAILED: {error}'}")

