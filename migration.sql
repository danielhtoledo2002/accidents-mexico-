drop database if exist Accidents;
create database Accidents;
use Accidents;
drop table if exists  accidentes_2020;
CREATE TABLE accidentes_2020(
    id integer NOT NULL AUTO_INCREMENT,
    COBERTURA VARCHAR(15) NOT NULL,
    ID_ENTIDAD varchar(5) not null,
    ID_MUNICIPIO varchar(5) not null,
    ANIO varchar(4) not null ,
    MES int(4) not null ,
    ID_HORA int(6) not null,
    ID_MINUTO int(5) not null,
    ID_DIA int(4) not null ,
    DIASEMANA varchar(50) not null ,
    URBANA varchar(100) not null ,
    SUBURBANA varchar(100) not null ,
    TIPACCID varchar(100) not null,
    AUTOMOVIL varchar(50) not null ,
    CAMPASAJ int(5) not null ,
    MICROBUS int(5) not null ,
    PASCAMION int(5) not null ,
    OMNIBUS int(5) not null ,
    TRANVIA int(5) not null ,
    CAMIONETA int(5) not null ,
    CAMION int(5) not null ,
    TRACTOR int(5) not null ,
    FERROCARRI int(5),
    MOTOCICLET int(5),
    BICICLETA int(5),
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

LOAD DATA  INFILE "/home/danielhtoledo/Accidentes/conjunto_de_datos/atus_anual_2020.csv"
INTO TABLE accidentes_2020
FIELDS TERMINATED BY ','
ENCLOSED BY '"'
LINES TERMINATED BY '\n'
IGNORE 1 ROWS;


drop table if exists  accidentes_2021;
CREATE TABLE accidentes_2021(
    id integer NOT NULL AUTO_INCREMENT,
    COBERTURA VARCHAR(15) NOT NULL,
    ID_ENTIDAD varchar(5) not null,
    ID_MUNICIPIO varchar(5) not null,
    ANIO varchar(4) not null ,
    MES int(4) not null ,
    ID_HORA int(6) not null,
    ID_MINUTO int(5) not null,
    ID_DIA int(4) not null ,
    DIASEMANA varchar(50) not null ,
    URBANA varchar(100) not null ,
    SUBURBANA varchar(100) not null ,
    TIPACCID varchar(100) not null,
    AUTOMOVIL varchar(50) not null ,
    CAMPASAJ int(5) not null ,
    MICROBUS int(5) not null ,
    PASCAMION int(5) not null ,
    OMNIBUS int(5) not null ,
    TRANVIA int(5) not null ,
    CAMIONETA int(5) not null ,
    CAMION int(5) not null ,
    TRACTOR int(5) not null ,
    FERROCARRI int(5),
    MOTOCICLET int(5),
    BICICLETA int(5),
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

LOAD DATA  INFILE "/home/danielhtoledo/Accidentes/conjunto_de_datos/atus_anual_2021.csv"
INTO TABLE accidentes_2021
FIELDS TERMINATED BY ','
ENCLOSED BY '"'
LINES TERMINATED BY '\n'
IGNORE 1 ROWS;