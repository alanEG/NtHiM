use serde::{Deserialize};

#[derive(Deserialize)]
struct Response {
	platforms: Vec<Definition>,
}

#[derive(Deserialize)]
struct Definition {
	platform: String,
	content: String,
}

pub fn _platforms(response: String) -> String {
	let _definitions = r#"
	{
		"platforms": [
		{
			"platform": "GitHub Pages",
			"content": "<p><strong>There isn't a GitHub Pages site here.</strong></p>"
		},
		{
			"platform": "Tumbler",
			"content": "Whatever you were looking for doesn't currently exist at this address."
		},
		{
			"platform": "WordPress.com",
			"content": "Do you want to register <em>"
		},
		{
			"platform": "TeamWork",
			"content": "Oops - We didn't find your site."
		},
		{
			"platform": "Helpscout",
			"content": "No settings were found for this company:"
		},
		{
			"platform": "Cargo",
			"content": "<title>404 &mdash; File not found</title>"
		},
		{
			"platform": "Uservoice",
			"content": "This UserVoice subdomain is currently available!"
		},
		{
			"platform": "Surge",
			"content": "project not found"
		},
		{
			"platform": "Webflow",
			"content": "<p class=\"description\">The page you are looking for doesn't exist or has been moved.</p>"
		},
		{
			"platform": "Kajabi",
			"content": "<h1>The page you were looking for doesn\'t exist.</h1>"
		},
		{
			"platform": "Thinkific",
			"content": "You may have mistyped the address or the page may have moved."
		},
		{
			"platform": "Tave",
			"content": "<h1>Error 404: Page Not Found</h1>"
		},
		{
			"platform": "Wishpond",
			"content": "<h1>https://www.wishpond.com/404?campaign=true"
		},
		{
			"platform": "Aftership",
			"content": "Oops.</h2><p class=\"text-muted text-tight\">The page you're looking for doesn't exist."
		},
		{
			"platform": "Aha",
			"content": "There is no portal here ... sending you back to Aha!"
		},
		{
			"platform": "Tictail",
			"content": "to target URL: <a href=\"https://tictail.com"
		},
		{
			"platform": "Tictail Second",
			"content": "Start selling on Tictail."
		},
		{
			"platform": "Brightcove",
			"content": "<p class=\"bc-gallery-error-code\">Error Code: 404</p>"
		},
		{
			"platform": "Bigcartel",
			"content": "<h1>Oops! We couldn&#8217;t find that page.</h1>"
		},
		{
			"platform": "ActiveCampaign",
			"content": "alt=\"LIGHTTPD - fly light.\""
		},
		{
			"platform": "Campaignmonitor",
			"content": "Double check the URL or <a href=\"mailto:help@createsend.com"
		},
		{
			"platform": "Amazon S3",
			"content": "<Code>NoSuchBucket</Code>"
		},
		{
			"platform": "Acquia",
			"content": "The site you are looking for could not be found."
		},
		{
			"platform": "Acquia Second",
			"content": "If you are an Acquia Cloud customer and expect to see your site at this address"
		},
		{
			"platform": "Proposify",
			"content": "If you need immediate assistance, please contact <a href=\"mailto:support@proposify.biz"
		},
		{
			"platform": "Simplebooklet",
			"content": "We can't find this <a href=\"https://simplebooklet.com"
		},
		{
			"platform": "GetResponse",
			"content": "With GetResponse Landing Pages, lead generation has never been easier"
		},
		{
			"platform": "Vend",
			"content": "Looks like you've traveled too far into cyberspace."
		},
		{
			"platform": "Jetbrains",
			"content": "is not a registered InCloud YouTrack."
		},
		{
			"platform": "Smartling",
			"content": "Domain is not configured"
		},
		{
			"platform": "Pingdom",
			"content": "pingdom"
		},
		{
			"platform": "Tilda",
			"content": "Domain has been assigned"
		},
		{
			"platform": "Mashery",
			"content": "Unrecognized domain <strong>"
		},
		{
			"platform": "Divio",
			"content": "Application not responding"
		},
		{
			"platform": "feedpress",
			"content": "The feed has not been found."
		},
		{
			"platform": "readme",
			"content": "Project doesnt exist... yet!"
		},
		{
			"platform": "statuspage",
			"content": "You are being <a href='https>"
		},
		{
			"platform": "zendesk",
			"content": "Help Center Closed"
		},
		{
			"platform": "zendesk",
			"content": "Hello! Sorry, but the webs>"
		},
		{
			"platform": "Agile CRM",
			"content": "Sorry, this page is no longer available."
		},
		{
			"platform": "Anima",
			"content": "If this is your website and you've just created it, try refreshing in a minute"
		},
		{
			"platform": "Bitbucket",
			"content": "Repository not found"
		},
		{
			"platform": "Campaign Monitor",
			"content": "Trying to access your account?"
		},
		{
			"platform": "DigitalOcean",
			"content": "Domain uses DO name serves with no records in DO."
		},
		{
			"platform": "Fastly",
			"content": "Fastly error: unknown domain:"
		},
		{
			"platform": "Ghost",
			"content": "The thing you were looking for is no longer here, or never was"
		},
		{
			"platform": "HatenaBlog",
			"content": "404 Blog is not found"
		},
		{
			"platform": "Help Juice",
			"content": "We could not find what you're looking for."
		},
		{
			"platform": "HelpScout",
			"content": "No settings were found for this company:"
		},
		{
			"platform": "Heroku",
			"content": "<title>No such app</title>"
		},
		{
			"platform": "Intercom",
			"content": "Uh oh. That page doesn't exist."
		},
		{
			"platform": "Intercom Second",
			"content": "This page is reserved for artistic dogs."
		},
		{
			"platform": "JetBrains YouTrack",
			"content": "is not a registered InCloud YouTrack"
		},
		{
			"platform": "Kinsta",
			"content": "You have successfully pointed this domain to Kinsta servers but you haven't added it to any of your sites."
		},
		{
			"platform": "LaunchRock",
			"content": "It looks like you may have taken a wrong turn somewhere. Don't worry...it happens to all of us."
		},
		{
			"platform": "Ngrok",
			"content": "Tunnel *.ngrok.io not found"
		},
		{
			"platform": "Pantheon",
			"content": "404 error unknown site!"
		},
		{
			"platform": "Pingdom",
			"content": "This public report page has not been activated by the user"
		},
		{
			"platform": "Readme.io",
			"content": "<h2>Project doesnt exist... yet!</h2>"
		},
		{
			"platform": "Shopify",
			"content": "<h1 class=\"tc\">Sorry, this shop is currently unavailable.</h1>"
		},
		{
			"platform": "SmartJobBoard",
			"content": "This job board website is either expired or its domain name is invalid."
		},
		{
			"platform": "Strikingly",
			"content": "<title>Page not found - Strikingly</title>"
		},
		{
			"platform": "Surge.sh",
			"content": "<h1>project not found</h1>"
		},
		{
			"platform": "Tumblr",
			"content": "Whatever you were looking for doesn't currently exist at this address. Unless you were looking for this error page, in which case: Congrats! You totally found it."
		},
		{
			"platform": "Uberflip",
			"content": "Non-hub domain, The URL you've accessed does not provide a hub."
		},
		{
			"platform": "Unbounce",
			"content": "The requested URL was not found on this server."
		},
		{
			"platform": "UserVoice",
			"content": "This UserVoice subdomain is currently available!"
		},
		{
			"platform": "Webflow",
			"content": "<p class=\"description\">The page you are looking for doesn't exist or has been moved.</p>"
		},
		{
			"platform": "Shopify",
			"content": "Sorry, this shop is currently unavailable."
		},
		{
			"platform": "Worksites",
			"content": "<p>Hello! Sorry, but the website you&rsquo;re looking for doesn&rsquo;t exist.</p>"
		},
		{
			"platform": "WordPress.com",
			"content": "<title>Error: Domain mapping upgrade for this domain not found</title>"
	
		}	
		]
	}"#;
	let data: Response = serde_json::from_str(_definitions).unwrap();
	let mut platformName: String = "None".to_string();
	for platform in data.platforms {
		if response.contains(&platform.content){
			platformName = platform.platform;
			break
		}
	}
	return platformName
}
