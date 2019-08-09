workflow "ci" {
  resolves = ["do-it"]
  on = "push"
}

action "do-it" {
  uses = "docker://rust:latest"
  runs = "cargo"
  args = "check"
}
