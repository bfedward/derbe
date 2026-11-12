CREATE TABLE role_accesses (
	unid uuid NOT NULL,
	created timestamptz NOT NULL,
	"role" text DEFAULT 'RegularUser'::text NOT NULL,
	granted_to_unid uuid NOT NULL,
	CONSTRAINT role_accesses_created_not_null NOT NULL created,
	CONSTRAINT role_accesses_granted_to_unid_not_null NOT NULL granted_to_unid,
	CONSTRAINT role_accesses_pkey PRIMARY KEY (unid),
	CONSTRAINT role_accesses_role_check CHECK ((role = ANY (ARRAY['Admin'::text, 'Manager'::text, 'RegularUser'::text, 'Demo'::text]))),
	CONSTRAINT role_accesses_role_not_null NOT NULL role,
	CONSTRAINT role_accesses_unid_not_null NOT NULL unid,
	CONSTRAINT role_accesses_granted_to_unid_fkey FOREIGN KEY (granted_to_unid) REFERENCES employees(unid)
);
CREATE INDEX idx_role_accesses_granted_to ON role_accesses USING btree (granted_to_unid);
CREATE INDEX idx_role_accesses_role ON role_accesses USING btree (role);