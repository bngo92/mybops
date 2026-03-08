use leptos::prelude::*;

pub fn docs() -> impl IntoView {
    crate::nav_content(
        view! {
          <a href="#" class="text-xl text-black">
            "Docs"
          </a>
        },
        view! {
          <div>
            <h1 class="mb-2 text-2xl font-medium">"Item features"</h1>
            <h2 class="text-xl font-medium">"Rank items"</h2>
            <p class="mb-2">
              "Rank items like your songs to figure out what your favorite songs are."
            </p>
            <h2 class="text-xl font-medium">"Rate items"</h2>
            <p class="mb-2">
              "Item ratings provide a method for ranking/grouping your items using a scale of 1 to 10."
            </p>
            <h2 class="text-xl font-medium">"Query items"</h2>
            <p class="mb-2">
              "Query using SQL to gain insights about your data by calculating statistics and filtering items.
               You can also view your data using different types of charts.
               The first column returned by the SQL query is used as the x-axis and the second column is used as the y."
            </p>
            <h2 class="text-xl font-medium">"Manage items"</h2>
            <p class="mb-4">
              "You can mark items as hidden for queries to filter on.
               You can also delete items to remove it from all lists and queries."
            </p>
            <h1 class="mb-2 text-2xl font-medium">"List features"</h1>
            <h2 class="text-xl font-medium">"Create lists of items using data sources"</h2>
            <p class="mb-2">
              "Add items to a list by adding a data source that resolves to items.
               Deleting a data source will remove the items from the list but the data for items will still be preserved."
            </p>
            <h2 class="text-xl font-medium">"Query items in a list"</h2>
            <p class="mb-2">
              "Queries under a list page are similar to queries in the top-level page except they will also be filtered against items in the list."
            </p>
            <h2 class="text-xl font-medium">"Integrate with external systems"</h2>
            <p class="mb-2">
              "If the data sources of your list support external integrations, you can define an ID that will be used for the integration.
               For example, lists that only use Spotify data sources can push the items into a Spotify playlist with the given ID."
            </p>
            <h2 class="text-xl font-medium">"Define a default query for the list"</h2>
            <p class="mb-2">
              "The query will be used as the default query for the query view and any push actions."
            </p>
            <h2 class="text-xl font-medium">"Favorite lists"</h2>
            <p class="mb-4">
              "Favorite lists will show up on the home page along with results from the default query."
            </p>
            <h1 class="mb-2 text-2xl font-medium">"Combined features"</h1>
            <p class="mb-2">"Create a Spotify playlist from other Spotify albums and playlists."</p>
            <p class="mb-2">"Create a Spotify playlist from songs that you've rated 7 or above."</p>
            <p class="mb-2">"Create a list from other lists to reuse the data sources."</p>
            <p class="mb-2">"Create a table showing your average ratings by album."</p>
            <p class="mb-2">
              "Create a column chart showing artists with the most number of songs you've rated 10 out of 10."
            </p>
          </div>
        },
    )
}
