import process
import visualization

def main():
    portfolio_data = process.import_wad_csv('../portfolio.csv')
    print(portfolio_data)

    visualization.plot_prices(portfolio_data)

if __name__ == "__main__":
    main()
