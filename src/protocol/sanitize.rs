pub(crate) fn sanitize(input: &str) -> String {
    input.trim().replace(" ", "").to_uppercase().to_string()
}
