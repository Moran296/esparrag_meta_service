use serde::{Deserialize, Serialize};
use serde_json::Value;

//used for testing
#[allow(dead_code)]
const SERVICE_1: &str = r#"{
  "service_name": "service_1",
  "description": "a test service",
  "actions": [
    {
      "action_name": "action_1",
      "description": "action 1 does something",
      "parameters": [
        {
          "param_name": "a_number_1",
          "description": "this number can be only positive and is required!",
          "type": "Uint32",
          "required": true
        },
        {
          "param_name": "a_number_2",
          "description": "this number can be positive and negative and is not required",
          "type": "Int32",
          "required": false,
          "default": "0"
        }
      ],
      "outputs": [
        {
          "param_name": "message",
          "description": "a message of success or failure",
          "type": {
            "Enum": [
              "ENUM_1",
              "ENUM_2"
            ]
          }
        }
      ]
    }
  ]
}"#;

/// paramters types of actions - serilizable as strings
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ParameterType {
    Bool,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Int8,
    Int16,
    Int32,
    Float,
    String,
    Enum(Vec<String>),
}

/// outputs of a possible action
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Output {
    pub param_name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_: ParameterType,
}

/// Parameters of a possible action
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub param_name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_: ParameterType,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

/// A service is a collection of actions.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub action_name: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
    pub outputs: Vec<Output>,
}

///Structure of a service API description which is serialized to JSON
/// Contains name, description and actions
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ServiceMeta {
    pub service_name: String,
    pub description: String,
    pub actions: Vec<Action>,
}

impl ServiceMeta {
    /// Creates a new service from a JSON string
    pub fn mock() -> ServiceMeta {
        serde_json::from_str(SERVICE_1).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn caters(&self, request: &Value) -> Result<(), &str> {
        let action = self.get_action(&request).ok_or("action not found")?;

        for parameter in action.parameters.iter() {
            if !self.caters_parameter(parameter, request) {
                return Err("Parameter not found");
            }
        }

        Ok(())
    }

    fn get_action(&self, request: &Value) -> Option<&Action> {
        if let Value::String(requested_action) = &request["action_name"] {
            if let Some(action) = self
                .actions
                .iter()
                .find(|action| *requested_action == action.action_name)
            {
                return Some(action);
            }
        }

        None
    }

    fn caters_parameter(&self, parameter: &Parameter, request: &Value) -> bool {
        if !parameter.required {
            return true;
        }

        if let Some(requested_parameter) = request.get(&parameter.param_name) {
            match &parameter.type_ {
                ParameterType::Uint8 => {
                    if let Some(value) = requested_parameter.as_u64() {
                        return value <= u8::max_value() as u64;
                    }
                }
                ParameterType::Uint16 => {
                    if let Some(value) = requested_parameter.as_u64() {
                        return value <= u16::max_value() as u64;
                    }
                }
                ParameterType::Uint32 => {
                    if let Some(value) = requested_parameter.as_u64() {
                        return value <= u32::max_value() as u64;
                    }
                }
                ParameterType::Uint64 => {
                    if let Some(value) = requested_parameter.as_u64() {
                        return value <= u32::max_value() as u64;
                    }
                }
                ParameterType::Int8 => {
                    if let Some(value) = requested_parameter.as_i64() {
                        return value <= i8::max_value() as i64;
                    }
                }
                ParameterType::Int16 => {
                    if let Some(value) = requested_parameter.as_i64() {
                        return value <= i16::max_value() as i64;
                    }
                }
                ParameterType::Int32 => {
                    if let Some(value) = requested_parameter.as_i64() {
                        return value <= i32::max_value() as i64;
                    }
                }
                ParameterType::Bool => return requested_parameter.is_boolean(),
                ParameterType::Float => return requested_parameter.is_f64(),
                ParameterType::String => return requested_parameter.is_string(),
                ParameterType::Enum(possibles) => {
                    if let Some(value) = requested_parameter.as_str() {
                        return possibles.contains(&value.to_string());
                    }
                }
            }
        }

        false
    }
}

//---------------- TESTING -------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_service() -> ServiceMeta {
        ServiceMeta {
            service_name: "service_1".to_string(),
            description: "a test service".to_string(),
            actions: vec![Action {
                action_name: "action_1".to_string(),
                description: "action 1 does something".to_string(),
                parameters: vec![
                    Parameter {
                        param_name: "a_number_1".to_string(),
                        description: "this number can be only positive and is required!"
                            .to_string(),
                        type_: ParameterType::Uint32,
                        required: true,
                        default: None,
                    },
                    Parameter {
                        param_name: "a_number_2".to_string(),
                        description: "this number can be positive and negative and is not required"
                            .to_string(),
                        type_: ParameterType::Int32,
                        required: false,
                        default: Some("0".to_string()),
                    },
                ],
                outputs: vec![Output {
                    param_name: "message".to_string(),
                    description: "a message of success or failure".to_string(),
                    type_: ParameterType::Enum(vec!["ENUM_1".to_string(), "ENUM_2".to_string()]),
                }],
            }],
        }
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn serialize_json() {
        let service = mock_service();
        let json = serde_json::to_string_pretty(&service).unwrap();
        println!("{}", json);
        assert_eq!(json, SERVICE_1.to_string());
    }

    #[test]
    fn deserialize_json() {
        let service = mock_service();
        let desirialized = serde_json::from_str(&SERVICE_1).unwrap();
        assert_eq!(service, desirialized);
    }

    #[test]
    fn caters_1() {
        let service = mock_service();
        let request = serde_json::from_str(
            r#"
        {
           "action_name": "action_1",
           "a_number_1": 33,
           "a_number_2": 42
        } "#,
        )
        .unwrap();

        assert!(service.caters(&request).is_ok());
    }

    #[test]
    fn caters_2() {
        let service = mock_service();
        let request = serde_json::from_str(
            r#"
        {
           "action_name": "action_1",
           "a_number_1": 33
        } "#,
        )
        .unwrap();

        assert!(service.caters(&request).is_ok());
    }

    #[test]
    fn not_caters_1() {
        let service = mock_service();
        let request = serde_json::from_str(
            r#"
        {
           "action_name": "action_1",
           "a_number_1": "33"
        } "#,
        )
        .unwrap();

        assert!(service.caters(&request).is_err());
    }

    #[test]
    fn not_caters_2() {
        let service = mock_service();
        let request = serde_json::from_str(
            r#"
        {
           "action_name": "action_4",
           "a_number_1": 33
        } "#,
        )
        .unwrap();

        assert!(service.caters(&request).is_err());
    }

    #[test]
    fn caters_enum() {
        let service = ServiceMeta {
            service_name: "service_1".to_string(),
            description: "a test service".to_string(),
            actions: vec![Action {
                action_name: "action_1".to_string(),
                description: "action 1 does something".to_string(),
                parameters: vec![Parameter {
                    param_name: "color".to_string(),
                    description: "this number can be only positive and is required!".to_string(),
                    type_: ParameterType::Enum(vec!["RED".to_string(), "BLUE".to_string()]),
                    required: true,
                    default: None,
                }],
                outputs: vec![Output {
                    param_name: "message".to_string(),
                    description: "a message of success or failure".to_string(),
                    type_: ParameterType::Enum(vec!["ENUM_1".to_string(), "ENUM_2".to_string()]),
                }],
            }],
        };

        let request = serde_json::from_str(
            r#"
        {
           "action_name": "action_1",
           "color": "RED"
        } "#,
        )
        .unwrap();

        assert!(service.caters(&request).is_ok());
    }

    #[test]
    fn not_caters_enum() {
        let service = ServiceMeta {
            service_name: "service_1".to_string(),
            description: "a test service".to_string(),
            actions: vec![Action {
                action_name: "action_1".to_string(),
                description: "action 1 does something".to_string(),
                parameters: vec![Parameter {
                    param_name: "color".to_string(),
                    description: "this number can be only positive and is required!".to_string(),
                    type_: ParameterType::Enum(vec!["RED".to_string(), "BLUE".to_string()]),
                    required: true,
                    default: None,
                }],
                outputs: vec![Output {
                    param_name: "message".to_string(),
                    description: "a message of success or failure".to_string(),
                    type_: ParameterType::Enum(vec!["ENUM_1".to_string(), "ENUM_2".to_string()]),
                }],
            }],
        };

        let request = serde_json::from_str(
            r#"
        {
           "action_name": "action_1",
           "color": "ORANGE"
        } "#,
        )
        .unwrap();

        assert!(service.caters(&request).is_err());
    }
}
