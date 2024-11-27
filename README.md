# rocket-league-ai-analysis

A command-line tool that analyzes Rocket League replay files using one or more AI services. This tool extracts meaningful insights, tactical analysis, and performance metrics from your replays.

## Features

- Parse Rocket League replay files and extract detailed game statistics and events
- Generate AI-powered analysis using a choice (or all) of ChatGPT, Claude, Gemini or Copilot APIs
- Provide tactical insights and improvement suggestions

## Prerequisites

- Rust (latest stable version)
- Claude, ChatGPT, Gemini and/or Copilot API keys
- Rocket League replay files

## Installation

1. Clone the repository:
```bash
git clone https://github.com/scottleedavis/rocket-league-ai-analysis.git
cd rocket-league-ai-analysis
```

2. Create a `.env` file in the project root:
```bash
CLAUDE_API_KEY=your-api-key-here
OPENAI_API_KEY=your-api-key-here
```

3. Build the project:

Locally
```bash
cargo build --release
```
and/or Docker
Locally
```bash
docker build -t rocket-league-replay-ai-analysis .
```

## Usage

1. Basic analysis:
```bash
cargo run -- analyze path/to/replay.replay
```
```bash
docker run rocket-league-replay-ai-analysis ...
```

2. Detailed analysis with specific focus:
```bash
cargo run -- analyze --focus tactical path/to/replay.replay
```
```bash
docker run rocket-league-replay-ai-analysis ...
```
## Configuration

The analyzer can be configured through command-line arguments or environment variables:

Claude
- `CLAUDE_API_KEY`: Your Claude API key
- `RL_ANALYZER_MODEL`: Claude model to use (default: "claude-3-sonnet-20240229")
- `RL_ANALYZER_LOG_LEVEL`: Log level (default: "info")
ChatGPT
-`OPENAI_API_KEY`: Your ChatGPT API key 
-
-
Gemini
- 
-
-
Copilot
-
-
-

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [rattletrap](https://github.com/tfausak/rattletrap) - Rocket League replay parser
- [rocketleague-replay-coach](https://github.com/scottleedavis/rocketleague-replay-coach) - ChatGPT proof of concept with python
- [Anthropic](https://anthropic.com) - Claude AI API