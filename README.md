# Introduction

This project was done in one semester of software engineering lessons. The aim was to provide an API for _something_, with atleast the CRUD (create, read, update, delete) operations as well as a health status. Anything done with the API should be represented in some sort of persistent storage.

Additionally, we should provide 2 Clients: one using [WPF](https://de.wikipedia.org/wiki/Windows_Presentation_Foundation), and another one using any web technology we want.

Following a tradition started by me and a few other students, I named this project after a slightly wrong written, rare first name: Kornelja.

# Tech Stack

My Tech stack was chosen based on ~~bad ideas~~ what would teach me the most.

As database I used [SurrealDB](https://surrealdb.com/), a _"next generation serverless database"_ written in Rust. It was actually quite interesting, and I can see myself using it in the future. SurrealQL provides several interesting features, among others permissions directly in the database as well as direct API access (which could have saved me some work if an API wasn't required 😅).

For my server I used [deno](https://deno.com/runtime), a _"modern runtime for JS and TS"_, again written in Rust. As language to write my server, I used the natively supported [typescript](https://www.typescriptlang.org/), a superset of javascript with really extensive type hinting. This was interesting to work with, as documentation was (compared to Node + express) limited, but I nonetheless hope to work with this again, as deno avoids huge node_modules.

One clients technology was given, as said I used WPF and C# as programming language

The other client was again very interesting. Seeing how most of my tech-stack was already written in Rust, i decided I should learn/write some rust myself. I used the new [leptos framework](https://github.com/leptos-rs/leptos), a web-framework for fine-grained reactivity, where you write in Rust. Luckily I could transfer some knowledge of react to at least the framework. Documentation was well, but (as before), there were few users asking questions on stackoverflow. Thanks to a welcoming discord server, I still found my way around.

# Installation & Setup

Following are several steps each for installation and afterwards deployment

## Installation

1. Clone this repo from github
1. Install SurrealDB ([link](https://surrealdb.com/install))
1. import all the surrealql files into your database (all the files in the [/DB/](https://github.com/euclid9099/messageboard/tree/main/DB)). Windows users like me can find a batch-file doing this one by one for you. Linux users can at least use the order in this, maybe you can use the file itself too.
1. Install deno ([link](https://deno.com/manual/getting_started/installation))
1. Install trunk ([link](https://leptos-rs.github.io/leptos/02_getting_started.html))
1. Install Visual Studio with .NET desktop development workload ([link](https://learn.microsoft.com/en-us/visualstudio/get-started/csharp/tutorial-wpf?view=vs-2022))

## Setup

1. start up SurrealDB (```surreal start --log trace --user <root> --pass <root> file:mydatabase.db```)
1. in [/server/helper.ts](https://github.com/euclid9099/messageboard/blob/main/server/helper.ts), change the db_url field to your db adress.
1. start up deno (```<project-root>/server# deno run app.ts```)
1. accept all connections deno request (one to surrealdb, one outgoing)
1. in [/leptos-client/lib.rs](https://github.com/euclid9099/messageboard/blob/main/leptos-client/src/lib.rs), change DEFAULT_API_URL to your API adress.
1. start up leptos (```<project-root>/leptos-client# trunk serve```)
1. in [/WPFClient/WPFClient/MainWindow.xaml.cs](https://github.com/euclid9099/messageboard/blob/main/WPFClient/WPFClient/MainWindow.xaml.cs), change api to your API adress.
1. launch the WPF client

# Usage

Once launched, you can use the WPF-client directly as windows executable. This client allows you to login, read posts and create posts. You can also like or dislike posts, and reload them at any time to show acurate impression-ratings or edited posts.

Via the website given in the terminal after ```trunk serve```, you can visit the webclient. This is intended to provide all the functionality, since it's easiest to distribute and doesn't require any download from sketchy sites.

The webclient allows you to both login with existing credentials, and register new accounts. Once logged in, you can logout at any time. Login tokens are stored, so as long as they are still valid (2h), they will be reloaded from cookies, even after leaving the site.

Most importantly, you can again view posts made by other users. There is also a button to create your own posts - no matter if you are logged in or not. You can also leave impressions on posts, in form of likes, dislikes (mutually exclusive), and responses. responses can be nested unlimited, though there is a reasonable limit in how responses can be represented.

the webclient also shows users. If you are logged in, you will see a tab "self" in your navigation bar. This leads to your own user. Even if not, you can click on any author of a post - these are links - and it will take you to their user page. Every user has a profile picture (link which has to end in one of the common image format extensions), an about me (limited to 250 characters) as well as followers and people they themselves are following. If you are logged in and viewing your own page, you can edit both profile picture and about text. Even when viewing some stranger, any logged in user can click a button to follow them.

## API

|Method|url|notes|body|response|Authorization (Token)|
|-|-|-|-|-|-|
|```GET```|```/```|health report, home page|None|HomeReply|no|
|```POST```|```/login```|login with username and password|```{"username": "<username>", "password": "<password>"}```|ApiReply\<ApiToken>|no|
|```POST```|```/signup```|register with username and password|```{"username": "<username>", "password": "<password>"}```|ApiReply\<ApiToken>|no|
||||||
|```GET```|```/posts```|get all posts|None|ApiReply<DBReply<Vec<Post>>>|no|
|```POST```|```/posts```|create a post|```{"message": "<post message>"}```|ApiReply<DBReply<Vec\<Post>>>|optional|
|```PATCH```|```/posts```|edit a post|```{"message": "<new message>"}```|ApiReply<DBReply<Vec\<Post>>>|required|
|```DELETE```|```/posts```|delete a post|None|ApiReply|required|
|```GET```|```/posts/<id>```|get one post|None|ApiReply<DBReply<Vec\<Post>>>|no|
|```POST```|```/posts/<id>/like```|like a post|None|ApiReply|required|
|```POST```|```/posts/<id>/dislike```|dislike a post|None|ApiReply|required|
||||||
|```GET```|```/users```|get all users|None|ApiReply<DBReply<Vec\<User>>>|no|
|```GET```|```/users/<id>```|get one users|None|ApiReply<DBReply<Vec\<User>>>|no|
|```PATCH```|```/users/<id>```|edit one user|Partial\<User>|ApiReply<DBReply<Vec\<User>>>|required|
|```DELETE```|```/users/<id>```|delete one user|None|ApiReply|required|
|```GET```|```/users/<id>/followers```|get all followers for one user|None|ApiReply<DBReply<Vec\<User>>>|no|
|```GET```|```/users/<id>/following```|get all users one user is following|None|ApiReply<DBReply<Vec\<User>>>|no|
|```POST```|```/users/<id>/follow```|follow one user|None|ApiReply|required|
|```POST```|```/users/<id>/unfollow```|unfollow one user|None|ApiReply|required|

## Models

ApiReply describes any reply from the API. message is either "ok" or "error". Depending on message, either content or error will be set.
```rs
struct ApiReply<T> {
    message: String,
    //either content or error are always set
    content: Option<T>,
    error: Option<Error>,
}
```

DBReply is the kind of reply any request returns, if it accesses SurrealDB.
```rs
enum DBReply<T> {
    OK { time: String, result: T },
    ERR { time: String, detail: String },
}
```

ApiToken is the main way of authentication used within the API. It's a JSON web token, generated by SurrealDB.
```rs
struct ApiToken {
    token: String
}
```

User is the datatype of any user (really surprising).
```rs
struct UserInfo {
    id: String,
    username: Option<String>,
    admin: bool, //defaults to false
    about: Option<String>,
    profile_picture: Option<String>,
    followers: Option<u32>,
    following: Option<u32>,
    you_follow: bool, //defaults to false
}
```

Post is the datatype of any post (I love remaining unpredictable).
```rs
struct Post {
    author: Option<UserInfo>,
    edited: bool,
    id: String,
    message: String,
    time: DateTime<Utc>,
    likes: u32,
    liked: Option<bool>,
    dislikes: u32,
    disliked: Option<bool>,
    responses: u16,
}
```

# Results

In total I was quite satisfied with my results. While the final results was not too overwhelming (especially in terms of design, truly not an artist), there were a lot of things I learned with this project. I am especially happy with having used SurrealDB. I like many concepts, and the functionality it provides all around. If possible, I'd like to work with it again in the future. There were also no big issues with any other of my components. I liked working with deno, for it has several nice security points, and native TypeScript support. Leptos is really fancy too, with how fast it (claims to be) is, and also some similiarities to other web-frameworks. I also wanted to use rust for quite some time, since it's a continiuously rising language, and most loved since many years.

I also feel like the project itself could be developed further, as the foundation is quite well done. Potential future ideas include:
- [ ] File upload for account icon and to include them in posts.
- [ ] using followers in some way, e.g. filtering posts for people you follow.
- [ ] integrating websockets for live-updating posts without constant manual reloading
- [ ] recommendation algorithms, instead of just ordered by time.
