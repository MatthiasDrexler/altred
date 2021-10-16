CREATE TABLE IF NOT EXISTS public."users"
(
    email character varying(254) COLLATE pg_catalog."default" NOT NULL,
    username character varying(40) COLLATE pg_catalog."default" NOT NULL,
    hashed_password character(60) COLLATE pg_catalog."default" NOT NULL,
    registration_date timestamp with time zone NOT NULL,
    activation_date timestamp with time zone,
    locked boolean NOT NULL DEFAULT false,
    CONSTRAINT user_pkey PRIMARY KEY (email)
);
