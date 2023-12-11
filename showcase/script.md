Hello everyone. I'm the developer of taxlint.
In this video, I'm going to show you what taxlint can do for you and how it works.

taxlint is a fully decentralized application on the internet computer, which aims to provide you with an easy way to manage and calculate your tax information related to your cryptocurrency activities.
First, let's log in to taxlint website.
Here I click the Launch APP button. And it's loading. Then I Choose my internet identity and input my password to login. After a few seconds.We can see the homepage.

on the homepage we can see dashboard and wallet.Now lets add  a wallet. For example, I input a test address and give it a name. Here I can also edit or delete my wallet. Now I'll show you edit my wallet.
And we click, update , Okay.
We can also delete the wallet. Click the delete button and it's good. It will be deleted after a few seconds.

After we add a wallet to taxlint and we click the address to see the detail. it shows the transaction type,Here, the amount of ICP ,and the profit we get.

The transaction records detail is grouped by month. You can see here November, five transactions, June, two transactions, May, three transactions.
And it shows a dashboard, a detailed transaction information on the ICP dashboard. Here we can click the export CSV to get the CSV file.

We can see the exported CSV file. It shows the send type, hash, time step when the transaction happens, and the amount of ICP and the fee, the price and the price at that time we get the profit from it. 

taxlint will automatically calculate the profit from the transaction and the transaction is get by the IC Rosetta API. Here we can see the code. And there is our code. We use Rosetta API to get the transactions. And we use FIFO as the tax calculation method. Here we're calculating the profit in the first in, first out method. You can check out the code on GitHub.

 taxlint is open source project. Any open source activity is welcome. Everyone is free to request for a new feature, report a bug, submit a pool request, and give feedback. That's all I want to share with you today. If you have any question or comments, please feel free to contact us. Thank you for watching.

```bash
ffmpeg -i AI_audio.mkv -vn -c:a libmp3lame output.mp3
```
- Who are your target users?
    Anyone who is need to report crypto tax ,such as  buy and sell between ICP and US dollar.
    In the process of generating capital gain and loss. If you need to report tax based on it. Using Taxlint can automatically calculate all profit or loss for you.


- What is the problem you are solving?
    User may encounter a tedious collection of transaction records, search for coin history value, compute detailed part of coin sale or buy, and finally calculate them on their own.
    We will automatically do most of the verbose work on our TaxLint dapp and generate tax report files for users.

- Why is this a problem (pain point) for your users?
    Calculating cryptocurrency tax is a complex work for users, using taxlint can make capital tax calculation much easier than doing it by hand every time.

- Why did you choose to build on IC?
    We agree with IC's philosophy of decentralized application, smart contracts, and self-governance by the community.  which makes the application code on IC safe and unbreakable.
