workflow "Do something on every pull request" {
  resolves = ["unit-test"]
  on = "push"
}

action "prepare" {
  uses = "docker://node:alpine"
  runs = "npm"
  args = "ci"
}

action "unit-test" {
  needs = "prepare"
  uses = "docker://node:alpine"
  runs = "npm"
  args = "test"
}
