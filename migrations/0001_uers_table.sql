CREATE TABLE users (
  id serial primary key,  
  username VARCHAR(255),
  password VARCHAR(255),
  firstname VARCHAR(255),
  lastname VARCHAR(255)
);

insert into users(username,password,firstname,lastname) values('myusername','mypassword','orlyn','gerano');
