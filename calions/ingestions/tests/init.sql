--
-- PostgreSQL database dump
--

-- Dumped from database version 13.1
-- Dumped by pg_dump version 13.1

-- Started on 2020-12-10 21:47:54 CET

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

CREATE TABLE public.ingested_contacts (
    id character(26) NOT NULL,
    data json NOT NULL,
    metadata json NOT NULL,
    organization_id uuid NOT NULL,
    inserted_at timestamp(0) without time zone NOT NULL,
    updated_at timestamp(0) without time zone NOT NULL
);


ALTER TABLE public.ingested_contacts OWNER TO rockysignavong;



ALTER TABLE ONLY public.ingested_contacts
    ADD CONSTRAINT ingested_contacts_pkey PRIMARY KEY (id);



CREATE INDEX ingested_contacts_organization_id_index ON public.ingested_contacts USING btree (organization_id);



CREATE TABLE public.gdpr_keys (
    id character(26) NOT NULL,
    contact_id character(26),
    data_group character varying(255) NOT NULL,
    key bytea NOT NULL,
    version integer DEFAULT 0 NOT NULL,
    algo character varying(60) NOT NULL,
    inserted_at timestamp(0) without time zone NOT NULL,
    updated_at timestamp(0) without time zone NOT NULL
);


ALTER TABLE public.gdpr_keys OWNER TO rockysignavong;


ALTER TABLE ONLY public.gdpr_keys
    ADD CONSTRAINT gdpr_keys_pkey PRIMARY KEY (id);



CREATE INDEX gdpr_keys_contact_id_data_group_index ON public.gdpr_keys USING btree (contact_id, data_group);



ALTER TABLE ONLY public.gdpr_keys
    ADD CONSTRAINT gdpr_keys_contact_id_fkey FOREIGN KEY (contact_id) REFERENCES public.ingested_contacts(id) ON DELETE CASCADE;


-- Completed on 2020-12-10 21:47:54 CET

--
-- PostgreSQL database dump complete
--

