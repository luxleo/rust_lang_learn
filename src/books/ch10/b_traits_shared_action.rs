/*
A trait defines the functionality a particular type has and can share with other types.
We can use traits to define shared behavior in an abstract way.
We can use trait bounds to specify that a generic type can be any type that has certain behavior.
*/
use std::hash::DefaultHasher;
use std::iter::Sum;

trait Summary {
    fn summarize_author(&self) -> String;

    // default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

trait Animal {
    fn breathe(&self) -> String;
}

struct NewsArticle {
    headline: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    // default implementation 도 overriding 가능하다.
    fn summarize(&self) -> String {
        format!("This news article is wrote by {}", self.summarize_author())
    }
}

struct LivingPaper {
    author: String,
    content: String,
}

impl Summary for LivingPaper {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Animal for LivingPaper {
    fn breathe(&self) -> String {
        format!("written by {}, and i feel so good", self.author)
    }
}

fn simulate_traits_and_struct() {
    let news = NewsArticle {
        author: String::from("dragon"),
        headline: String::from("title"),
        content: String::from("this is content"),
    };

    println!("{}", news.summarize());
}

//Section : trait as parameter

fn do_breathe_v1(animal : &impl Animal){
    println!("{}", animal.breathe());
}

// using 'Trait Bound' syntax  same as above
fn do_breathe_v2<T: Animal>(animal: T) {
    println!("{}", animal.breathe());
}

// &impl cannot restrict same type but 'Trait Bound' can
fn cannot_restrict_same_type(item1: &impl Summary, item2: &impl Summary) {
    println!("item1: {} item2: {}", item1.summarize(),item2.summarize());
}

fn can_restrict_same_type<T: Summary>(item1: &T, item2: &T) {
    println!("item1: {} item2: {}", item1.summarize(),item2.summarize());
}

fn simulate_trait() {
    let living_docs = LivingPaper {
        author: String::from("author1"),
        content: String::from("content1"),
    };
    let dead_docs =  NewsArticle {
        author: String::from("dragon"),
        headline: String::from("title"),
        content: String::from("this is content"),
    };
    cannot_restrict_same_type(&living_docs, &dead_docs);
}

// Specifying Multiple Trait Bounds with the + Syntax
fn only_living_docs_v1(docs: &(impl Summary + Animal)) {
    println!("{}", docs.breathe());
}

fn only_living_docs_v2<T: Animal + Summary>(docs: T) {
    println!("{}", docs.breathe());
}

// clearer trait bounds with where clauses
fn dirty_signature<T: Animal + Summary, U: Summary>(item1: T, item2: U) {
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
}

fn readable_signature<T,U>(item1: T, item2: U)
where
    T: Animal + Summary,
    U: Summary
{
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
}

//Returning Types That Implement Traits
fn get_templated_living_doc() -> impl Summary + Animal {
    LivingPaper { author: String::from("author1"), content: String::from("content1"), }
}

//Using Trait Bounds to Conditionally Implement Methods
struct MysteryBox<T>{
    doc : T
}

impl<T> MysteryBox<T>{
    fn new(doc : T) -> Self{
        Self{doc}
    }
}

impl<T: Summary + Animal> MysteryBox<T>{
    fn yes_iam_living_docs(&self){
        println!("yes iam rare living documents {}", self.doc.breathe());
    }
}

fn simulate_conditional_method() {
    let normal_box = MysteryBox {
        doc: NewsArticle {
            author: String::from("dragon"),
            headline: String::from("title"),
            content: String::from("this is content"),
        }
    };
    // ^^^^^^^^^^^^^^^^^^^ method cannot be called on `MysteryBox<NewsArticle>` due to unsatisfied trait bounds
    // normal_box.yes_iam_living_docs();

    let rare_box = MysteryBox {
        doc: LivingPaper { author: String::from("author1"), content: String::from("content1") }
    };
    rare_box.yes_iam_living_docs();
}

fn main(){
    simulate_traits_and_struct();
    simulate_trait();
    simulate_conditional_method();
}