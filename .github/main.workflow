workflow "ci" {
  resolves = ["do-it"]
  on = "push"
}

action "do-it" {
  uses = "docker://circleci/rust"
  runs = "cargo"
  args = "check"
}
