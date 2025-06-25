WITH inserted_group AS (INSERT INTO "group" ("name", shortname, kind, parent) VALUES ('admin', 'A', 'role', null) RETURNING id)
INSERT INTO "role"(name, permissions, "group")
VALUES ('admin',
('[
{"subject": "user", "ops": 15, "ids": null},
{"subject": "role", "ops": 15, "ids": null},
{"subject": "group", "ops": 15, "ids": null},
{"subject": "file", "ops": 15, "ids": null},
{"subject": "course", "ops": 15, "ids": null},
{"subject": "template", "ops": 15, "ids": null}
]'::json),
(SELECT id FROM inserted_group)
)
RETURNING id;

INSERT INTO user_permissions (role_id, permission) VALUES((SELECT id FROM "role" WHERE name = 'admin'), 15::bit(16));
INSERT INTO role_permissions (role_id, permission) VALUES((SELECT id FROM "role" WHERE name = 'admin'), 15::bit(16));
INSERT INTO group_permissions (role_id, permission) VALUES((SELECT id FROM "role" WHERE name = 'admin'), 15::bit(16));
INSERT INTO file_permissions (role_id, permission) VALUES((SELECT id FROM "role" WHERE name = 'admin'), 15::bit(16));
INSERT INTO course_permissions (role_id, permission) VALUES((SELECT id FROM "role" WHERE name = 'admin'), 15::bit(16));
INSERT INTO template_permissions (role_id, permission) VALUES((SELECT id FROM "role" WHERE name = 'admin'), 15::bit(16));
