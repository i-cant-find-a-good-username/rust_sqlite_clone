create table users(
    id integer auto_increment,
    username text not_null,
    password text not_null
);

insert into users values('dude', 'password')