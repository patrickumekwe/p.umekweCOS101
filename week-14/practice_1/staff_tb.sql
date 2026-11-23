--
-- PostgreSQL database dump
--

\restrict j2XeqgJMMbnFZ8eVheKJ0JmkKiqeAeffSl5LrVyGvJRlk9jYLmYBMtHhXJF8Cbk

-- Dumped from database version 18.6
-- Dumped by pg_dump version 18.6

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: PCU
--

CREATE TABLE public.staff (
    staff_id integer CONSTRAINT employees_eid_not_null NOT NULL,
    staff_name character varying(50) CONSTRAINT employees_ename_not_null NOT NULL,
    dno integer CONSTRAINT employees_dno_not_null NOT NULL,
    staff_sal integer CONSTRAINT employees_esal_not_null NOT NULL,
    age integer CONSTRAINT employees_age_not_null NOT NULL,
    mobile character varying(11)
);


ALTER TABLE public.staff OWNER TO "PCU";

--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: PCU
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
1	Kayode	1	120000	17	08031234501
2	Zion	1	95000	16	08031234502
3	Patrick	2	110000	18	08031234503
4	Dieko	2	85000	20	07031234504
5	Kareem	3	70000	15	09031234505
6	Tumi	3	130000	19	08131234506
7	Michelle	4	140000	17	08031234507
8	Chikere	4	105000	20	07031234508
9	Adaugo	1	100000	18	09031234509
\.


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: PCU
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

\unrestrict j2XeqgJMMbnFZ8eVheKJ0JmkKiqeAeffSl5LrVyGvJRlk9jYLmYBMtHhXJF8Cbk

