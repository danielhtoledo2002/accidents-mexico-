import mysql.connector as msql
from mysql.connector import Error
try:
    conn = mysql.connect(host='localhost', database='accidentes', user='root', password='1234')
    if conn.is_connected():
        cursor = conn.cursor()
        cursor.execute("select database();")
        record = cursor.fetchone()
        print("You're connected to database: ", record)
        cursor.execute('DROP TABLE IF EXISTS employee_data;')
        print('Creating table....')
        cursor.execute("CREATE TABLE accidentes_2019(Id int(10) unsigned NOT NULL AUTO_INCREMENT, COBERTURA varchar(255) NOT NULL,ID_ENTIDAD int(4) unsigned NOT NULL, company_name varchar(255),address varchar(255),city varchar(255),county varchar(255),state varchar(255),zip int,phone1 varchar(255),phone2 varchar(255),email varchar(255),web varchar(255))")
        print("Table is created....")
        for i,row in empdata.iterrows():
            sql = "INSERT INTO employee.employee_data VALUES (%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s,%s)"
            cursor.execute(sql, tuple(row))
            print("Record inserted")
            conn.commit()
except Error as e:
            print("Error while connecting to MySQL", e)
