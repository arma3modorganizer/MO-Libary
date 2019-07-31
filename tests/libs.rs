#[cfg(test)]
mod tests {
    use a3mo_lib::repository::new;
    use a3mo_lib::repository::build;
    #[test]
    //TODO
    fn test_create_repository(){
        assert!(new::new("TestModPack", "D:\\___Arma3Sync\\test_mod_pack", "Url",  true).is_ok());
    }
}