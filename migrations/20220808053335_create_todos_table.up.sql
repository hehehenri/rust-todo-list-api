CREATE TABLE todos (
	id int NOT NULL AUTO_INCREMENT,
    name varchar(255) NOT NULL,
    is_checked bool DEFAULT 0,
    PRIMARY KEY (id)
);