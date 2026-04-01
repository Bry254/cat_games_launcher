# CAT GAMES LAUNCHER
Launcher simple para lanzar juegos de linux creado en rust con egui pensado para ser sencillo de utilizar.
---
> ⚠️ **Build experimental (beta)**
> 
> Esta aplicación se encuentra en fase **beta (experimental)**.  
> Pueden existir errores o comportamientos inesperados.

## Instalación
Descarga el archivo de los `releases`, configuralo para que se ejecute como programa y dale click.

## Compilacion
- Escribe en terminal (Nesesitas tener [rust](https://www.rust-lang.org/) y [git](https://git-scm.com/install/linux) instalado en tu sistema) (solo compila la interfaz):
```bash
git clone https://github.com/Bry254/cat_games_launcher && cd cat_games_launcher && cargo build --release && cp ./target/release/cat_games_launcher ./
```
Esto te soltara el launcher `cat_games_launcher` en la carpeta solo haz click en el.

## Caracteristicas
- Ejecuta juego en linux (Por defecto `linux` y `wine`).
- Crear Atajos al menu inicio de tus juegos.
- Configuraciones que puedes desactivar y activar por juegos.
- Exportar la configuración de tu juego en un archivo .cat_game para despues poderlo importar.
- Puedes crear tus propios runners (por ejemplo añadir la opcion de poder jugar con `proton` )
- Detectado de icono del juego (si utilizas juegos .exe podrias instalar `wrestool` ó `icoextract` para que el programa obtenga sus iconos automaticamente)
