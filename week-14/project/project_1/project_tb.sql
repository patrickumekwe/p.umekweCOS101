--
-- PostgreSQL database dump
--

\restrict zYfxONdGh4i9o7fUlFPAzFcoh5JTgWaX8XhLW5OGQlkXGTvluGHhHq0ZpgWOTia

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
-- Name: projects; Type: TABLE; Schema: public; Owner: PCU
--

CREATE TABLE public.projects (
    pno integer NOT NULL,
    pname character varying(40) NOT NULL,
    pduration real NOT NULL,
    pmanager_id integer
);


ALTER TABLE public.projects OWNER TO "PCU";

--
-- Data for Name: projects; Type: TABLE DATA; Schema: public; Owner: PCU
--

COPY public.projects (pno, pname, pduration, pmanager_id) FROM stdin;
101	Budget Tracker	0.6666	1
102	Student Survey	0.3333	3
103	Brand Campaign	1	6
104	Prototype Build	1.3333	7
105	Annual Report	2	9
\.


--
-- Name: projects projects_pkey; Type: CONSTRAINT; Schema: public; Owner: PCU
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_pkey PRIMARY KEY (pno);


--
-- PostgreSQL database dump complete
--

\unrestrict zYfxONdGh4i9o7fUlFPAzFcoh5JTgWaX8XhLW5OGQlkXGTvluGHhHq0ZpgWOTia

