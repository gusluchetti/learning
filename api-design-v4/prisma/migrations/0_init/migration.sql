-- CreateTable
CREATE TABLE "comments" (
    "cid" SERIAL NOT NULL,
    "post_id" INTEGER NOT NULL,
    "comment" VARCHAR(255),
    "date_created" TIMESTAMP(6),

    CONSTRAINT "comments_pkey" PRIMARY KEY ("cid")
);

-- CreateTable
CREATE TABLE "posts" (
    "pid" SERIAL NOT NULL,
    "title" VARCHAR(255),
    "body" VARCHAR,
    "created_at" TIMESTAMP(6) DEFAULT CURRENT_TIMESTAMP,
    "likes" INTEGER DEFAULT 0,

    CONSTRAINT "posts_pkey" PRIMARY KEY ("pid")
);

-- CreateTable
CREATE TABLE "users" (
    "uid" UUID NOT NULL DEFAULT gen_random_uuid(),
    "username" VARCHAR(32) NOT NULL,

    CONSTRAINT "users_pkey" PRIMARY KEY ("uid")
);

-- AddForeignKey
ALTER TABLE "comments" ADD CONSTRAINT "fk_post" FOREIGN KEY ("post_id") REFERENCES "posts"("pid") ON DELETE NO ACTION ON UPDATE NO ACTION;

