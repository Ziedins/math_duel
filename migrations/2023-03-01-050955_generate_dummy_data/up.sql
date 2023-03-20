-- Your SQL goes here
INSERT INTO users(id, username, created_at)
VALUES
("4fbd288c-d3b2-4f78-adcf-def976902d50","klavs","2022-11-23T07:56:30.214162+00:00"),
("1e9a12c1-e98c-4a83-a55a-32cc548a169d","reinis","2022-11-23T07:56:30.214162+00:00");

INSERT INTO games(id, name, user_a_id, user_b_id, active_user_id, goal_a, goal_b,initial_value, current_value, status, created_at)
VALUES
("f061383b-0393-4ce8-9a85-f31d03762263", "Demo1", "4fbd288c-d3b2-4f78-adcf-def976902d50", "1e9a12c1-e98c-4a83-a55a-32cc548a169d", "4fbd288c-d3b2-4f78-adcf-def976902d50", 19, 10, 4, 1, "turn_a", "2022-12-23T07:56:30.214162+00:00"),
("008e9dc4-f01d-4429-ba31-986d7e63cce8", "Demo2", "1e9a12c1-e98c-4a83-a55a-32cc548a169d" , "4fbd288c-d3b2-4f78-adcf-def976902d50", "4fbd288c-d3b2-4f78-adcf-def976902d50", 3, 5, 3, 2, "turn_b", "2022-12-23T07:56:30.214162+00:00");

INSERT INTO moves(id, user_id, game_id, operator, term, created_at)
VALUES
("9aeab1a7-e063-40d1-a120-1f7585fa47d6", "4fbd288c-d3b2-4f78-adcf-def976902d50", "f061383b-0393-4ce8-9a85-f31d03762263","+", "3", "2022-12-23T07:56:30.214162+00:00"),
("9aeab1a7-e063-40d1-a120-1f7585fa47d7", "1e9a12c1-e98c-4a83-a55a-32cc548a169d", "f061383b-0393-4ce8-9a85-f31d03762263","*", "2", "2022-12-23T07:57:30.214162+00:00"),
("9aeab1a7-e063-40d1-a120-1f7585fa47d8", "4fbd288c-d3b2-4f78-adcf-def976902d50", "f061383b-0393-4ce8-9a85-f31d03762263","-", "10", "2022-12-23T07:57:45.214162+00:00");
