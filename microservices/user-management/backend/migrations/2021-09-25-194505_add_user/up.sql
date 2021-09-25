CREATE TABLE IF NOT EXISTS public."user"
(
    email character varying(254) COLLATE pg_catalog."default" NOT NULL,
    username character varying(40) COLLATE pg_catalog."default" NOT NULL,
    hashed_password character(60) COLLATE pg_catalog."default" NOT NULL,
    registration_date date NOT NULL,
    activation_date date,
    locked boolean NOT NULL DEFAULT false,
    CONSTRAINT user_pkey PRIMARY KEY (email)
);

ALTER TABLE public."user"
    OWNER to local;
