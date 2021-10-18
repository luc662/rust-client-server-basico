use std::io::{Read, Write};

#[derive(Debug)]
pub struct Usuario {
    pub nombre: String,
    pub contrasenia: String
}

impl Usuario {
    pub fn write_to(&self, stream: &mut dyn Write) -> std::io::Result<()> {

        let mut size_be = (self.contrasenia.len() as u32).to_be_bytes();
        stream.write(&size_be)?;
        let contrasenia_be = self.contrasenia.as_bytes();
        stream.write(&contrasenia_be)?;
        // Es importante aclarar el tipo de variable de len(), sino será usize
        size_be = (self.nombre.len() as u32).to_be_bytes();
        stream.write(&size_be)?;
        stream.write(&self.nombre.as_bytes())?;
        Ok(())
    }

    pub fn read_from(stream: &mut dyn Read) -> std::io::Result<Usuario> {
        let mut num_buffer = [0u8; 4];
        // Leo 4 bytes, con longitud
        stream.read_exact(&mut num_buffer)?;
        let mut size = u32::from_be_bytes(num_buffer);
        let mut contrasenia_buf = vec![0; size as usize];
        stream.read_exact(&mut contrasenia_buf)?;
        // Convierto de bytes a string.
        let contrasenia_str = std::str::from_utf8(&contrasenia_buf).expect("Error al leer nombre");
        stream.read_exact(&mut num_buffer)?;
        // Una vez que leemos los bytes, los convertimos a un u32
        size = u32::from_be_bytes(num_buffer);
        // Creamos un buffer para el nombre
        let mut nombre_buf = vec![0; size as usize];
        stream.read_exact(&mut nombre_buf)?;
        // Convierto de bytes a string.
        let nombre_str = std::str::from_utf8(&nombre_buf).expect("Error al leer nombre");
        let nombre = nombre_str.to_owned();
        let contrasenia = contrasenia_str.to_owned();
        let usuario = Usuario { nombre, contrasenia };
        Ok(usuario)
    }
}
