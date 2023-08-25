import process
import visualization

def main():
    portfolio_data = process.import_wad_csv('../portfolio.csv')
    visualization.plot_all(portfolio_data, 'combined_plot.png')

if __name__ == "__main__":
    main()
