import argparse
import process
import visualization
import statistical_visualization

def main():
    parser = argparse.ArgumentParser(description='Run different types of visualizations.')
    parser.add_argument('--type', type=str, choices=['statistical', 'single'], required=True,
                        help='Type of visualization to run. Choose "statistical" for mean and std, "single" for individual runs.')

    args = parser.parse_args()

    if args.type == 'statistical':
        simulation_data = process.import_all_csvs('../output')
        [means, stds] = process.compute_mean_and_std(simulation_data)
        statistical_visualization.plot_all(means.astype(float), stds.astype(float), 'combined_plot_statistical.png')
    elif args.type == 'single':
        simulation_data = process.import_wad_csv('../output/portfolio_0.csv')
        print(simulation_data['arbitrageur_relative_balances_x'])
        print(simulation_data['arbitrageur_relative_balances_y'])
        visualization.plot_all(simulation_data, 'combined_plot_single.png')

if __name__ == "__main__":
    main()


if __name__ == "__main__":
    main()
