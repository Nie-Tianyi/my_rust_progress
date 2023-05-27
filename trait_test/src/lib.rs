pub mod aggregator{
    //define a trait
    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String{
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle{
        pub headline:String,
        pub location:String,
        pub author:String,
        pub content:String,
    }

    impl Summary for NewsArticle{
        fn summarize(&self) -> String {
            format!("{},by {} {}",self.headline,self.author,self.location)
        }

        fn summarize_author(&self) -> String {
            String::from("impl of summarize_author")
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }

        fn summarize_author(&self) -> String {
            String::from("impl of summarize_author")
        }


    }
    //该参数是实现了 Summary trait 的某种类型
    // pub fn notify(item:&impl Summary){
    //     println!("Breaking news! {}", item.summarize());
    // }

    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    //pub fn notify(item: &(impl Summary + Display))
    //pub fn notify<T: Summary + Display>(item: &T)

    //fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32
    //使用 where 从句
    /*

    fn some_function<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        unimplemented!()
    }



    */
    //返回实现了某个 trait 的类型
    fn returns_summarizable() -> impl Summary {
        //返回类型必须得确定
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }



}