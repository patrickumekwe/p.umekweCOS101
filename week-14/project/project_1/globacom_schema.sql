--
-- PostgreSQL database dump
--

\restrict sk8nmU8hLBO5Zffi5zfsPtJlDjSlwK5xHQCJUIEmXnSbNUc14Ub2cKwcKZ3JZDe

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
-- Name: customer; Type: TABLE; Schema: public; Owner: PCU
--

CREATE TABLE public.customer (
    c_id integer NOT NULL,
    c_name character varying(50) NOT NULL,
    c_age integer NOT NULL,
    c_email character varying(80) NOT NULL,
    c_mobile character varying(15) NOT NULL,
    eid integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer OWNER TO "PCU";

--
-- Name: dataplan; Type: TABLE; Schema: public; Owner: PCU
--

CREATE TABLE public.dataplan (
    data_id integer NOT NULL,
    data_size character varying(10) NOT NULL,
    data_duration integer NOT NULL,
    data_price integer NOT NULL
);


ALTER TABLE public.dataplan OWNER TO "PCU";

--
-- Name: departments; Type: TABLE; Schema: public; Owner: PCU
--

CREATE TABLE public.departments (
    dno integer NOT NULL,
    dname character varying(50) NOT NULL,
    dept_managerid integer,
    dlocation character varying(50) NOT NULL,
    pno integer NOT NULL
);


ALTER TABLE public.departments OWNER TO "PCU";

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
-- Data for Name: customer; Type: TABLE DATA; Schema: public; Owner: PCU
--

COPY public.customer (c_id, c_name, c_age, c_email, c_mobile, eid, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	08055089112	102	5
111	Lilian Jaiya	43	l_jaiye@gmail.com	08055185341	100	3
112	Arthur Musa	50	a_musa@gmail.com	07055282813	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com	09052356772	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	08053333551	120	5
115	Oghenero Agor	50	o_agor@gmail.com	07055566774	117	11
116	Adams Bree	33	a_bree@gmail.com	08056765424	102	1
117	Okafor Mathias	45	o_mathias@gmail.com	08056763367	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com	07056774423	117	11
119	Lawal Tamire	35	l_tamire@gmail.com	09052111101	107	5
120	James Job	44	j_job@gmail.com	08059693919	100	8
121	Matthew Jakande	21	m_jakande@gmail.com	07051232144	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	08054921923	107	5
\.


--
-- Data for Name: dataplan; Type: TABLE DATA; Schema: public; Owner: PCU
--

COPY public.dataplan (data_id, data_size, data_duration, data_price) FROM stdin;
1	350MB	2	200
2	1.8GB	14	500
3	3.9GB	30	1000
4	7.5GB	30	1500
5	9.2GB	30	2000
6	10.8GB	30	2500
7	14GB	30	3000
8	18GB	30	4000
9	24GB	30	5000
10	29.9GB	30	8000
11	50GB	30	10000
\.


--
-- Data for Name: departments; Type: TABLE DATA; Schema: public; Owner: PCU
--

COPY public.departments (dno, dname, dept_managerid, dlocation, pno) FROM stdin;
1	Finance	1	Room 1	101
2	Business Admin	3	Room 3	102
3	Marketing & Sales	6	Eko Foyer	103
4	Research & Development	7	Saperto Block	104
\.


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
-- Name: customer customer_pkey; Type: CONSTRAINT; Schema: public; Owner: PCU
--

ALTER TABLE ONLY public.customer
    ADD CONSTRAINT customer_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan dataplan_pkey; Type: CONSTRAINT; Schema: public; Owner: PCU
--

ALTER TABLE ONLY public.dataplan
    ADD CONSTRAINT dataplan_pkey PRIMARY KEY (data_id);


--
-- Name: departments departments_pkey; Type: CONSTRAINT; Schema: public; Owner: PCU
--

ALTER TABLE ONLY public.departments
    ADD CONSTRAINT departments_pkey PRIMARY KEY (dno);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: PCU
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: projects projects_pkey; Type: CONSTRAINT; Schema: public; Owner: PCU
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_pkey PRIMARY KEY (pno);


--
-- PostgreSQL database dump complete
--

\unrestrict sk8nmU8hLBO5Zffi5zfsPtJlDjSlwK5xHQCJUIEmXnSbNUc14Ub2cKwcKZ3JZDe

