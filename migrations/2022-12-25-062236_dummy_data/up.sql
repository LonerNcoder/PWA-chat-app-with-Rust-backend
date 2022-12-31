-- Your SQL goes here
INSERT INTO users(id, username, phone, password, created_at) 
VALUES
("4fbd288c-d3b2-4f78-adcf-def976902d50","Parag Sarkar","123", "12345678","2022-11-23T07:56:30.214162+00:00"),
("1e9a12c1-e98c-4a83-a55a-32cc548a169d","Bipul Sarkar","345","12345678","2022-11-23T07:56:30.214162+00:00"),
("1bc833808-05ed-455a-9d26-64fe1d96d62d","Ritwik Saha","678","12345678","2022-12-23T07:56:30.214162+00:00");

INSERT INTO rooms(room_id, last_message, created_at)
VALUES
("4fbd288c-d3b2-4f78-adcf-def976902d50&1bc833808-05ed-455a-9d26-64fe1d96d62d", "Hi, how are you?","2022-12-23T07:56:30.214162+00:00");


INSERT INTO conversations(id, sender_id, receiver_id, room_id, content,seen, created_at)
VALUES
("9aeab1a7-e063-40d1-a120-1f7585fa47d6", "1bc833808-05ed-455a-9d26-64fe1d96d62d","4fbd288c-d3b2-4f78-adcf-def976902d50", "4fbd288c-d3b2-4f78-adcf-def976902d50&1bc833808-05ed-455a-9d26-64fe1d96d62d", "Hello",false, "2022-12-23T07:56:30.214162+00:00"),
("f4e54e70-736b-4a79-a622-3659b0b555e8", "4fbd288c-d3b2-4f78-adcf-def976902d50","1bc833808-05ed-455a-9d26-64fe1d96d62d", "4fbd288c-d3b2-4f78-adcf-def976902d50&1bc833808-05ed-455a-9d26-64fe1d96d62d", "Hi, how are you?",false, "2022-12-24T07:56:30.214162+00:00");
