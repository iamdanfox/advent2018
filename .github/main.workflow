workflow "Do something on every pull request" {
  resolves = ["prepare"]
  on = "push"
}

action "prepare" {
  uses = "docker://node:alpine"
  runs = "find"
  args = "."
}
