-- Your SQL goes here
create table places
(
    id          serial primary key,
    name        varchar not null,
    lat         float   not null,
    lng         float   not null,
    description varchar not null,
    address     varchar not null,
    img_url     varchar not null default 'no_image'
)