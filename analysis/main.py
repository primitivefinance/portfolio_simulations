import process
import statistical_visualization

def main():
    simulation_data = process.import_all_csvs('../output')
    [means, stds] = process.compute_mean_and_std(simulation_data)
    statistical_visualization.plot_all(means.astype(float), stds.astype(float), 'combined_plot.png')

if __name__ == "__main__":
    main()
