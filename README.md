# Yuki
A command line chat bot based on OpenAI's ChatGPT-3

# config.yaml

The format for config.yaml

```yaml
# your OpenAI api_key
api_key: xxx-xxxxxx

# the dir to store your session for next use, empty mean not save
sessions_dir: 'path/to/sessions'
# the dir to store your role
roles_dir: 'path/to/roles'

# the default session when boot(UUID), empty mean no default
default_session: '2029b196-ddab-4666-8e25-9f544485a938'


```

# role.yaml

The format for role.yaml, which stores your conversation properties with chat model. These file name are <uuid>.yaml.

```yaml
# the role's uuid generate by random
role_id: '105f5c60-823a-4a39-b3f7-cd45c40346ed'
# the short name for this role
name: 'ChatGpt'

# some model settings ref. https://platform.openai.com/docs/api-reference/chat 
model:
  # [0~2]
  temperature: 0.8
  # [0~1]
  top_P: 1
  # max:4096 ,use -1 for inf (4096-prompt)
  max_tokens: -1
  # [-2~2]
  presence_penalty: 0
  # [-2~2]
  frequency_penalty: 0
  # a map for token-value
  logit_bias:

# the prompt for this role, prompt is important to customize a role, which will nerver be truncated.
prompt: "You are a helpful assistant."

```

# session.json

The format for session.json, which stores your conversation record with chat mode. These file name are <uuid>.json

```json
{
  "session_id": "2029b196-ddab-4666-8e25-9f544485a938",
  "role_id": "105f5c60-823a-4a39-b3f7-cd45c40346ed",
  "conversations": [
    {
      "role": "user",
      "content": "Hello"
    },
    {
      "role": "assistant",
      "content": "Hello! How may I assist you today?"
    }
  ]
}

```
