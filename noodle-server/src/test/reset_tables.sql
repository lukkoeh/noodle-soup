TRUNCATE TABLE "user", "group", "role" RESTART IDENTITY CASCADE;
TRUNCATE TABLE "user_permissions";
TRUNCATE TABLE "role_permissions";
TRUNCATE TABLE "group_permissions";
TRUNCATE TABLE "file_in_content_element" RESTART IDENTITY CASCADE;
TRUNCATE TABLE "content_element" RESTART IDENTITY CASCADE;
TRUNCATE TABLE "content_section" RESTART IDENTITY CASCADE;
TRUNCATE TABLE "template" RESTART IDENTITY CASCADE;
TRUNCATE TABLE "course" RESTART IDENTITY CASCADE;
TRUNCATE TABLE "file" RESTART IDENTITY CASCADE;
