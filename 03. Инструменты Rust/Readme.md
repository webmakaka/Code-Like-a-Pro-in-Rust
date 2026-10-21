# 03. Инструменты Rust

<br/>

### Основной инструментарий языка Rust

| Название      | Описание                                                                                                                                                             |
| :------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Cargo         | Инструмент управления Rust-проектами, предназначенный для компиляции, тестирования и управления зависимостями (рассматривается в главе 2)                            |
| Rust-analyzer | Обеспечивает поддержку Rust для текстовых редакторов, реализующих протокол языкового сервера (Language Server Protocol, LSP)                                         |
| Rustfmt       | Инструмент категоричного задания стиля написания кода Rust, обеспечивающий автоматическое форматирование и проверку кода и способный интегрироваться в системы CI/CD |
| Clippy        | Инструмент обеспечения качества кода Rust, выполняющий множество проверок (называемых _линтами_) и способный интегрироваться в системы CI/CD                         |
| Sccache       | Универсальный инструмент кеширования компилятора для повышения скорости компиляции в крупных проектах                                                                |

<br/>

### Установка rust-analyzer

<br/>

```shell
$ code --install-extension rust-lang.rust-analyzer
```

<br/>

### Установка rustfmt

<br/>

```shell
$ rustup component add rustfmt
```

<br/>

```shell
$ cargo fmt
# Теперь задействованный вами код будет отформатирован с помощью rustfmt
```

<br/>

### Установка Clippy

<br/>

```shell
$ rustup component add clippy
```

<br/>

```shell
$ cargo clippy
```

<br/>

### Установка Sccache

<br/>

```shell
$ cargo install sccache
```

<br/>

```shell
$ export RUSTC_WRAPPER='which sccache'
$ cargo build
```

Здесь команда which sccache возвращает путь к sccache, предполагая, что он доступен В $РАТН.

<br/>

### Интеграция Rust с Visual Studio Code

<br/>

```shell
$ rustup component add rust-src
```

<br/>

```shell
$ code --install-extension matklad.rust-analyzer
```

<br/>

### Помержание пакетов в актуальном состоянии: инструмент cargo-update

<br/>

```shell
$ cargo install cargo-update

// Обновление всех установленных пакетов
$ cargo cargo-update -а
```

<br/>

### Отладка макросов: инструмент cargo-expand

<br/>

```shell
$ cargo install cargo-expand
$ cargo expand outennod: :innennod
```

<br/>

### Тестирование с применением libfuzzer

<br/>

```shell
$ cargo install cargo-fuzz
```

<br/>

```shell
$ cargo fuzz new myfuzztest
$ cargo fuzz run myfuzztest
```

<br/>

### Периодический запуск Саrgо-команд: инструмент cargo-watch

<br/>

```shell
$ cargo install cargo-watch
$ cargo watch
```

<br/>

### Проверка зависимостей: инструмент cargo-tree

<br/>

```shell
$ cargo install cargo-tree
$ cargo tree
```
