CREATE TABLE "posts" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "description" TEXT NOT NULL,
    "user_id" UUID NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT "fk_user_id"
        FOREIGN KEY ("user_id")
        REFERENCES "users"("id")
);

CREATE TABLE "categories" (
    "id" UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    "name" TEXT NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE posts_categories (
    post_id UUID NOT NULL,
    category_id UUID NOT NULL,

    CONSTRAINT "pk_posts_categories"
        PRIMARY KEY (post_id, category_id),

    CONSTRAINT "fk_post_id"
        FOREIGN KEY (post_id)
        REFERENCES "posts"("id"),

    CONSTRAINT "fk_category_id"
        FOREIGN KEY (category_id)
        REFERENCES "categories"("id")
);
