CREATE TABLE category_site
(
    category_id INT NOT NULL REFERENCES category (id),
    site_id     INT NOT NULL REFERENCES site (id)
);
