CREATE TABLE notifications (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	user_id UUID NOT NULL,
	title TEXT NOT NULL,
	body TEXT NOT NULL,
	read BOOLEAN NOT NULL DEFAULT false,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_notifications_user_id
	ON notifications (user_id);

CREATE INDEX idx_notifications_user_unread
	ON notifications (user_id)
	WHERE read = false;
