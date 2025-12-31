
CREATE TABLE caustic_user (
    user_id TEXT PRIMARY KEY,
    email TEXT NOT NULL,
    created TEXT NOT NULL
);

CREATE TABLE caustic_project (
    project_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_user_id TEXT NOT NULL,
    created TEXT NOT NULL,
    last_modified TEXT NOT NULL,
    FOREIGN KEY (owner_user_id) REFERENCES caustic_user(user_id)
);

CREATE TABLE caustic_project_file (
    project_id TEXT NOT NULL,
    filename TEXT NOT NULL,
    content_type TEXT NOT NULL,
    created TEXT NOT NULL,
    last_modified TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES caustic_project(project_id),
    PRIMARY KEY (project_id, filename)
);

INSERT INTO caustic_user (user_id, email, created) VALUES ('examples', '', '2025-12-30T22:31:50.922Z');

-- Example: Car
INSERT INTO caustic_project
    (project_id, name, owner_user_id, created, last_modified)
VALUES
    ('cad84577-c808-41a9-8d77-25a4626fe65f', 'Example: Car', 'examples', '2025-12-30T22:31:50.922Z', '2025-12-30T22:31:50.922Z');

INSERT INTO caustic_project_file
    (project_id, content_type, filename, created, last_modified)
VALUES
    ('cad84577-c808-41a9-8d77-25a4626fe65f', 'application/x-openscad', 'car.scad', '2025-12-30T22:31:50.922Z', '2025-12-30T22:31:50.922Z');

-- Example: Random Spheres
INSERT INTO caustic_project
    (project_id, name, owner_user_id, created, last_modified)
VALUES
    ('cb50f13d-c3ea-41da-9369-ca73728f0808', 'Example: Random Spheres', 'examples', '2025-12-30T22:31:50.922Z', '2025-12-30T22:31:50.922Z');

INSERT INTO caustic_project_file
    (project_id, content_type, filename, created, last_modified)
VALUES
    ('cb50f13d-c3ea-41da-9369-ca73728f0808', 'application/x-openscad', 'random-spheres.scad', '2025-12-30T22:31:50.922Z', '2025-12-30T22:31:50.922Z');

-- Example: Three Spheres
INSERT INTO caustic_project
    (project_id, name, owner_user_id, created, last_modified)
VALUES
    ('b43378fe-afa5-4706-aa09-0951ff1564f2', 'Example: Three Spheres', 'examples', '2025-12-30T22:31:50.922Z', '2025-12-30T22:31:50.922Z');

INSERT INTO caustic_project_file
    (project_id, content_type, filename, created, last_modified)
VALUES
    ('b43378fe-afa5-4706-aa09-0951ff1564f2', 'application/x-openscad', 'three-spheres.scad', '2025-12-30T22:31:50.922Z', '2025-12-30T22:31:50.922Z');
