use canyon_sql::*;
// use chrono::NaiveDate;
pub mod league;
pub mod tournament;

use chrono::NaiveDate;
use league::*;
use tournament::*;

/// The `#[canyon]` macro represents the entry point of a Canyon program.
/// 
/// When this annotation it's present, Canyon it's able to take care about everything
/// for you related to mantain the database that you provide in the `secrets.toml` file,
/// being the most obvious and important the migrations control.
#[canyon]
fn main() {
    /*  
        The insert example.
        On the first run, you may desire to uncomment the method call below,
        to be able to populate some data into the schema.
        Remember that all operation with CanyonCrud must be awaited,
        due to it's inherent async nature
    */
    // _wire_data_on_schema().await;

    /*
        The most basic usage pattern.
        Finds all elements on a type T, if the type its annotated with the
        #[derive(Debug, Clone, CanyonCrud, CanyonMapper)] derive macro

        This automatically returns a collection (Vector) of elements found
        after query the database, automatically desearializating the returning
        rows into elements of type T
    */

    // Move into example of multi_insert()
    let _all_leagues: Vec<League> = League::find_all().await;
    println!("Leagues elements: {:?}", &_all_leagues);

    let new_league = League {
        id: 10,
        ext_id: 392489032,
        slug: "League10".to_owned(),
        name: "League10also".to_owned(),
        region: "Turkey".to_owned(),
        image_url: "https://www.sdklafjsd.com".to_owned()
    };
    let new_league2 = League {
        id: 0,
        ext_id: 392489032,
        slug: "League11".to_owned(),
        name: "League11also".to_owned(),
        region: "LDASKJF".to_owned(),
        image_url: "https://www.sdklafjsd.com".to_owned()
    };
    let new_league3 = League {
        id: 3,
        ext_id: 9687392489032,
        slug: "League3".to_owned(),
        name: "3League".to_owned(),
        region: "EU".to_owned(),
        image_url: "https://www.lag.com".to_owned()
    };

    League::insert_into(
        &[new_league, new_league2, new_league3]
    ).await;

    /*
        Canyon also has a powerful querybuilder.
        Every associated function or method provided through the macro implementations
        that returns a QueryBuilder type can be used as a raw builder to construct
        the query that Canyon will use to retrive data from the database.

        One really important thing to note it's that any struct annotated with the
        `[#canyon_entity]` annotation will automatically generates two enumerations
        for the curren type, following the convention: 
        
        Type identifier + Field, holding variants to identify every
        field that the type has. You can recover the field name by writting:
        `Type::variant.field_name_as_str()`

        Type identifier + FieldValue, holding variants to identify every
        field that the type has and let the user attach them data of the same
        data type that the field is bounded.
        You can recover the passed in value when created by writting:
        `Type::variant(some_value).value()` that will gives you access to the
        some value inside the variant.

        So for a -> 
            pub struct League { /* fields */ }
        an enum with the fields as variants its generated ->
            pub enum LeagueField { /* variants */ }
        an enum with the fields as variants its generated ->
            pub enum LeagueFieldValue { /* variants(data_type) */ }

        So you must bring into scope `use::/* path to my type .rs file */::TypeFieldValue`
        or simply `use::/* path to my type .rs file */::*` with a wildcard import.
        
        The querybuilder methods usually accept one of the variants of the enum to make a filter
        for the SQL clause, and a variant of the Canyon's `Comp` enum type, which indicates
        how the comparation element on the filter clauses will be 
    */
    let _all_leagues_as_querybuilder: Vec<League> = League::find_all_query()
        .where_clause(
            LeagueFieldValue::id(1), // This will create a filter -> `WHERE type.id = 1`
            Comp::Eq // where the `=` symbol it's given by this variant
        )
        .query()
        .await;
    println!("Leagues elements QUERYBUILDER: {:?}", &_all_leagues_as_querybuilder);

    // Quick example on how to update multiple columns on a table
    // This concrete example will update the columns slug and image_url with
    // the provided values to all the entries on the League table which ID
    // is greater than 3
    League::update_query()
        .set_clause(
            &[
                (LeagueField::slug, "Updated slug"),
                (LeagueField::image_url, "https://random_updated_url.up")
            ]
        ).where_clause(
            LeagueFieldValue::id(3), Comp::Gt
        );
        // Remove the semicolon and add the lines below if you want to try the update
        // .query()
        // .await;

    // Uncomment to see the example of find by a Fk relation
    _search_data_by_fk_example().await;
}

/// Example of usage of the `.insert()` Crud operation. Also, allows you
/// to wire some data on the database to be able to retrieve and play with data 
/// 
/// Notice how the `fn` must be `async`, due to Canyon's usage of **tokio**
/// as it's runtime
/// 
/// One big important note on Canyon insert. Canyon automatically manages
/// the ID field (commonly the primary key of any table) for you.
/// This means that if you keep calling this method, Canyon will keep inserting
/// records on the database, not with the id on the instance, only with the 
/// autogenerated one. 
/// 
/// This may change on a nearly time. 'cause it's direct implications on the
/// data integrity, but for now keep an eye on this.
/// 
/// An example of multiples inserts ignoring the provided `id` could end on a
/// situation like this:
/// 
/// ```
/// ... League { id: 43, ext_id: 1, slug: "LEC", name: "League Europe Champions", region: "EU West", image_url: "https://lec.eu" }, 
/// League { id: 44, ext_id: 2, slug: "LCK", name: "League Champions Korea", region: "South Korea", image_url: "https://korean_lck.kr" }, 
/// League { id: 45, ext_id: 1, slug: "LEC", name: "League Europe Champions", region: "EU West", image_url: "https://lec.eu" }, 
/// League { id: 46, ext_id: 2, slug: "LCK", name: "League Champions Korea", region: "South Korea", image_url: "https://korean_lck.kr" } ...
/// ``` 
async fn _wire_data_on_schema() {
    // Data for the examples
    let lec: League = League {
        id: 1,
        ext_id: 1,
        slug: "LEC".to_string(),
        name: "League Europe Champions".to_string(),
        region: "EU West".to_string(),
        image_url: "https://lec.eu".to_string(),
    };

    let lck: League = League {
        id: 2,
        ext_id: 2,
        slug: "LCK".to_string(),
        name: "League Champions Korea".to_string(),
        region: "South Korea".to_string(),
        image_url: "https://korean_lck.kr".to_string(),
    };

    // Now, the insert operations in Canyon is designed as a method over
    // the object, so the data of the instance is automatically parsed
    // into it's correct types and formats and inserted into the table
    lec.insert().await;
    lck.insert().await;

    /*  At some point on the console, if the operation it's successful, 
        you must see something similar to this, depending on the logging
        level choosed on Canyon
        
        INSERT STMT: INSERT INTO leagues (ext_id, slug, name, region, image_url) VALUES ($1,$2,$3,$4,$5)
        FIELDS: id, ext_id, slug, name, region, image_url

        INSERT STMT: INSERT INTO leagues (ext_id, slug, name, region, image_url) VALUES ($1,$2,$3,$4,$5)
        FIELDS: id, ext_id, slug, name, region, image_url
    */
}

/// Example of usage for a search given an entity related throught the 
/// `ForeignKey` annotation
/// 
/// Every struct that contains a `ForeignKey` annotation will have automatically
/// implemented a method to find data by an entity that it's related
/// through a foreign key relation.
/// 
/// So, in the example, the struct `Tournament` has a `ForeignKey` annotation
/// in it's `league` field, which holds a value relating the data on the `id` column
/// on the table `League`, so Canyon will generate an associated function following the convenction
/// `Type::search_by__name_of_the_related_table` 
/// 
/// TODO Upgrade DOCS according the two new ways of perform the fk search
async fn _search_data_by_fk_example() {
    // TODO Care with the docs. Split in two examples the two fk ways

    // TODO Explain that Canyon let's you annotate an entity with a FK but until a query, we 
    // can't no secure that the parent really exists
    
    // Basic example of 'insert' when you have a FK relation
    // let lpl: League = League {
    //     id: 25, // TODO This is not the real ID. The ID it's always AUTOGENERATED, so it's
    //             // irrelevant this number here. This point will be upgraded at some moment
    //             // in the future
    //     ext_id: 1,
    //     slug: "LPL".to_string(),
    //     name: "League of Legends PRO League".to_string(),
    //     region: "China".to_string(),
    //     image_url: "https://lpl.china".to_string(),
    // };
    // lpl.insert().await;

    // So, to recover the lpl related data in DB, and workaround the AUTOGENERATED ID, 
    // we can make a query by some other field and get the ID
    let some_lpl: Vec<League> = League::find_all_query()
        .where_clause(
            LeagueFieldValue::slug("LPL".to_string()),  // This will create a filter -> `WHERE type.slug = "LPL"`
            Comp::Eq  // where the `=` symbol it's given by this variant
        )
        .query()
        .await;
    println!("LPL QUERYBUILDER: {:?}", &some_lpl);
        

    let tournament_itce = Tournament {
        id: 1,
        ext_id: 4126494859789,
        slug: "Slugaso".to_string(),
        start_date: NaiveDate::from_ymd(2022, 5, 07),
        end_date: NaiveDate::from_ymd(2023, 5, 10),
        league: some_lpl
            .get(0)  // Returns an Option<&T>
            .cloned()
            .unwrap()
            .id,  // The Foreign Key, pointing to the table 'League' and the 'id' column
    };
    tournament_itce.insert().await;

    // You can search the 'League' that it's the parent of 'Tournament'
    let related_tournaments_league_method: Option<League> = tournament_itce.search_league().await;
    println!("The related League as method: {:?}", &related_tournaments_league_method);

    // Also, the common usage w'd be operating on data retrieve from the database, `but find_by_id`
    // returns an Option<T>, so an Option destructurement should be necessary
    let tournament: Option<Tournament> = Tournament::find_by_id(1).await;
    println!("Tournament: {:?}", &tournament);

    if let Some(trnmt) = tournament {
        let result: Option<League> = trnmt.search_league().await;
        println!("The related League as method if tournament is some: {:?}", &result);
    } else { println!("`tournament` variable contains a None value") }
    
    // The alternative as an associated function, passing as argument a type <K: ForeignKeyable> 
    // Data for the examples. Obviously will also work passing the above `tournament` variable as argument
    let lec: League = League {
        id: 4,
        ext_id: 1,
        slug: "LEC".to_string(),
        name: "League Europe Champions".to_string(),
        region: "EU West".to_string(),
        image_url: "https://lec.eu".to_string(),
    };
    let related_tournaments_league: Option<League> = Tournament::belongs_to(&lec).await;
    println!("The related League as associated function: {:?}", &related_tournaments_league);

    let tournaments_belongs_to_league: Vec<Tournament> = Tournament::search_by__league(&lec).await;
    println!("Tournament belongs to a league: {:?}", &tournaments_belongs_to_league);
}