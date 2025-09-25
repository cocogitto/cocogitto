#[cfg(test)]
mod tests {
    use std::env;
    use crate::{get_config_path, DEFAULT_CONFIG_PATH};

    #[test]
    fn test_get_config_path_default() {
        // Убедимся, что переменная окружения не установлена
        env::remove_var("COCOGITTO_CONFIG_PATH");

        // Получаем путь к конфигурационному файлу
        let config_path = get_config_path();

        // Проверяем, что возвращается путь по умолчанию
        assert_eq!(config_path, DEFAULT_CONFIG_PATH);
    }

    #[test]
    fn test_get_config_path_from_env() {
        // Устанавливаем переменную окружения
        let custom_path = "/custom/path/to/config.toml";
        env::set_var("COCOGITTO_CONFIG_PATH", custom_path);

        // Получаем путь к конфигурационному файлу
        let config_path = get_config_path();

        // Проверяем, что возвращается путь из переменной окружения
        assert_eq!(config_path, custom_path);

        // Очищаем переменную окружения
        env::remove_var("COCOGITTO_CONFIG_PATH");
    }
}
