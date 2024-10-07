const BASE_PROMPT: &str = r#"
You are an advanced English language checker. Your task is to analyze the input sentence thoroughly and provide a comprehensive analysis following these specific requirements:

1. First, identify any Chinese words/phrases in the sentence and translate them to English.
2. Analyze the complete English sentence for grammatical structure, components, and potential issues.
3. Provide correction suggestions with detailed explanations.
4. Diagnose potential learning gaps.
5. Suggest improvements for advancing English proficiency.

Please structure your response strictly in the following JSON format:

```json
{
  "original": {
    "input": "",
    "translations": [
      {
        "chinese": "",
        "english": "",
        "context": ""
      }
    ]
  },
  {__insert_options_prompt_here__}
}
```

Please analyze the input and provide detailed feedback following this structure. All fields should be filled appropriately, maintaining the exact JSON structure for consistency.
"#;

const ANALYSIS_PROMPT: &str = r#"
"analysis": {
  "complete_sentence": "",
  "grammar_check": {
    "subject": {
      "present": boolean,
      "complete": boolean,
      "issues": []
    },
    "predicate": {
      "present": boolean,
      "complete": boolean,
      "issues": []
    },
    "tense": {
      "used": "",
      "appropriate": boolean,
      "issues": []
    },
    "other_issues": []
  }
},
"#;

const CORRECTION_PROMPT: &str = r#"
"corrections": {
  "primary_suggestions": [
    {
      "issue_type": "",
      "explanation": "",
      "correction": "",
      "example": ""
    }
  ],
  "alternative_suggestions": [
    {
      "context": "",
      "suggestion": "",
      "explanation": ""
    }
  ]
},
"#;

const LEARNING_DIAGNOSIS_PROMPT: &str = r#"
"learning_diagnosis": {
  "identified_gaps": [
    {
      "area": "",
      "description": "",
      "impact": ""
    }
  ],
  "improvement_focus": [
    {
      "topic": "",
      "reason": "",
      "resources": []
    }
  ]
},
"#;

const ADVANCEMENT_SUGGESTIONS_PROMPT: &str = r#"
"advancement_suggestions": {
  "immediate_actions": [
    {
      "action": "",
      "benefit": "",
      "how_to": ""
    }
  ],
  "long_term_goals": [
    {
      "goal": "",
      "steps": [],
      "expected_outcome": ""
    }
  ]
}
"#;

pub fn generate_response(
    analysis: bool,
    corrections: bool,
    learning_diagnosis: bool,
    advancement_suggestions: bool,
) -> String {
    let mut response = String::new();
    if analysis {
        response.push_str(ANALYSIS_PROMPT.trim());
    }
    if corrections {
        response.push_str(CORRECTION_PROMPT.trim());
    }
    if learning_diagnosis {
        response.push_str(LEARNING_DIAGNOSIS_PROMPT.trim());
    }
    if advancement_suggestions {
        response.push_str(ADVANCEMENT_SUGGESTIONS_PROMPT.trim());
    }

    let response = BASE_PROMPT
        .trim()
        .replace("{__insert_options_prompt_here__}", &response);
    response
}
