use Accidents;
drop table if exists  accidentes_2021;
CREATE TABLE accidentes_2021(
    id integer NOT NULL ,
    COBERTURA VARCHAR(15) NOT NULL,
    ID_ENTIDAD varchar(5) not null,
    ID_MUNICIPIO varchar(5) not null,
    ANIO varchar(4) not null ,
    MES int(4) not null ,
    ID_HORA varchar(10) not null,
    ID_MINUTO varchar(5) not null,
    ID_DIA varchar(4) not null ,
    DIASEMANA varchar(50) not null ,
    URBANA varchar(100) not null ,
    SUBURBANA varchar(100) not null ,
    TIPACCID varchar(100) not null,
    AUTOMOVIL varchar(50) not null ,
    CAMPASAJ varchar(50) not null ,
    MICROBUS varchar(50) not null ,
    PASCAMION varchar(50) not null ,
    OMNIBUS varchar(50) not null ,
    TRANVIA varchar(50) not null ,
    CAMIONETA varchar(50) not null ,
    CAMION varchar(50) not null ,
    TRACTOR varchar(50) not null ,
    FERROCARRI varchar(50),
    MOTOCICLET varchar(50),
    BICICLETA varchar(50),
    OTROVEHIC int(5),
    CAUSAACCI varchar(100),
    CAPAROD varchar(100),
    SEXO varchar(100),
    ALIENTO varchar(100),
    CINTURON varchar(100),
    ID_EDAD varchar(5),
    CONDMUERTO varchar(50),
    CONDHERIDO varchar(50),
    PASAMUERTO varchar(50),
    PASAHERIDO varchar(50),
    PEATMUERTO varchar(50),
    PEATHERIDO varchar(50),
    CICLMUERTO varchar(50),
    CICLHERIDO varchar(50),
    OTROMUERTO varchar(50),
    OTROHERIDO varchar(50),
    NEMUERTO varchar(50),
    NEHERIDO varchar(50),
    CLASACC varchar(70),
    ESTATUS varchar(100),
    PRIMARY KEY (id)
);


LOAD DATA  INFILE "/var/lib/mysql-files/atus_anual_2021.csv"
INTO TABLE accidentes_2021
FIELDS TERMINATED BY ','
ENCLOSED BY '"'
LINES TERMINATED BY '\n'
IGNORE 1 ROWS;