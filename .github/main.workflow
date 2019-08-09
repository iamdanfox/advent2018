workflow "Do something on every pull request" {
  resolves = ["./action-a"]
  on = "pull_request"
}

action "./action-a" {
  uses = "./action-a"
  secrets = ["GITHUB_TOKEN"]
}
