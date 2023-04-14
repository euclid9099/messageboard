using Microsoft.IdentityModel.Tokens;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.IdentityModel.Tokens.Jwt;
using System.Linq;
using System.Net.Http;
using System.Net.Http.Json;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;

namespace WPFClient
{
    /// <summary>
    /// Interaction logic for MainWindow.xaml
    /// </summary>
    public partial class MainWindow : Window
    {
        public static string api = "http://localhost:7700";

        public JwtSecurityToken? token;
        private bool accountActive;
        private DateTime mostRecentPost;
        public static HttpClient httpClient;

        public MainWindow()
        {
            InitializeComponent();
            httpClient = new HttpClient();
            accountActive = false;
            mostRecentPost = DateTime.MinValue;
        }

        private void ActivateAccount(Object sender, RoutedEventArgs e)
        {
            accountActive = true;
            accountDisplay.Fill = new SolidColorBrush(Colors.LimeGreen);
            httpClient.DefaultRequestHeaders.Add("x-token", this.token?.RawData);
        }

        private void DeactivateAccount(Object sender, RoutedEventArgs e)
        {
            accountActive = false;
            accountDisplay.Fill = new SolidColorBrush(Colors.Gray);
            httpClient.DefaultRequestHeaders.Remove("x-token");
        }

        private void Login(object sender, RoutedEventArgs e)
        {
            string uname = usernameTextbox.Text;
            if (string.IsNullOrEmpty(uname)) return;

            string body = $"{{\"username\": \"{uname}\",\"password\": \"{passwordTextbox.Password}\"}}";
            var response = httpClient.PostAsync($"{api}/login", new StringContent(body, Encoding.UTF8, "application/json")).Result;
            var resBody = response.Content.ReadFromJsonAsync<Response<AuthorizeResponse>>().Result;

            if (resBody?.Content == null)
            {
                MessageBox.Show(resBody?.Error?.Message, resBody?.Error?.Name, MessageBoxButton.OK, MessageBoxImage.Error);
                return;
            }

            this.token = new JwtSecurityToken(resBody.Content.Token);

            accountTab.Text = $"Account ({uname})";

            useAccountCheckbox.IsChecked = true; //also executes ActivateAccount

            loginView.Visibility = Visibility.Hidden;
            accountView.Visibility = Visibility.Visible;
        }

        private void Logout(object sender, RoutedEventArgs e)
        {
            DeactivateAccount(sender, e);

            accountTab.Text = "Account (anonymus)";

            accountView.Visibility = Visibility.Hidden;
            loginView.Visibility = Visibility.Visible;
        }

        private void LoadMorePostsButton(Object sender, RoutedEventArgs e)
        {
            var response = httpClient.GetFromJsonAsync<Response<DBResponse<List<PostResponse>>>>($"{api}/posts?after={mostRecentPost.ToString("o", CultureInfo.InvariantCulture)}{(!accountActive || token == null || !token.Payload.ContainsKey("ID") ? "" : "&as=" + token.Payload["ID"].ToString())}").Result;

            if (response?.Content == null || response.Content.Result.Count == 0) return;
            
            foreach (var post in response.Content.Result)
            {
                postsListbox.Items.Add(post);
            }
            mostRecentPost = response.Content.Result.Last().Time;
        }

        private void ReloadPost(object sender, RoutedEventArgs e)
        {
            var item = (ListBoxItem)postsListbox.ContainerFromElement((Button)sender);
            item.IsSelected = true;
            var response = httpClient.GetFromJsonAsync<Response<DBResponse<List<PostResponse>>>>($"{api}/posts/{((PostResponse)postsListbox.Items[postsListbox.SelectedIndex]).Id}{(!accountActive || token == null || !token.Payload.ContainsKey("ID") ? "" : "?as=" + token.Payload["ID"].ToString())}").Result;

            if (response?.Content == null || response.Content.Result.Count == 0) return;
            postsListbox.Items[postsListbox.SelectedIndex] = response.Content.Result.First();
        }

        private void ChangeImpressionOnPost(object sender, RoutedEventArgs e, bool onLike)
        {
            if (!accountActive)
            {
                MessageBox.Show("Cannot like post without active account", "OnO", MessageBoxButton.OK, MessageBoxImage.Error);
                return;
            }

            var listitem = (ListBoxItem)postsListbox.ContainerFromElement((Button)sender);
            var post = (PostResponse)listitem.Content;

            if (post.Liked == null || post.Disliked == null)
            {
                MessageBox.Show("Reload post to retrieve account specific data", "OnO", MessageBoxButton.OK, MessageBoxImage.Error);
                return;
            }
            _ = httpClient.PostAsync($"{api}/posts/{post.Id}/{(onLike ? ((bool)post.Liked ? "like?reset" : "like") : ((bool)post.Disliked ? "dislike?reset" : "dislike"))}", null).Result;
            ReloadPost(sender, e);
        }

        private void LikePost(object sender, RoutedEventArgs e)
        {
            ChangeImpressionOnPost(sender, e, true);
        }

        private void DislikePost(object sender, RoutedEventArgs e)
        {
            ChangeImpressionOnPost(sender, e, false);
        }

        private void CreateNewPost(object sender, RoutedEventArgs e)
        {
            if (newPostContent.Text.IsNullOrEmpty()) return;
            _ = httpClient.PostAsync($"{api}/posts", new StringContent($"{{\"message\":\"{newPostContent.Text}\"}}", Encoding.UTF8, "application/json")).Result;
            LoadMorePostsButton(sender, e);
            newPostContent.Text = "";
        }
    }
}
